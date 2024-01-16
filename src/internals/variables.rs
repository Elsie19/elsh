use core::fmt;
use std::collections::HashMap;
use std::fs;
use std::ops::Add;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum Type {
    String(String),
    Integer(u32),
    Float(f32),
    Array(Vec<Type>),
}

#[derive(Debug, Clone)]
pub struct VariableStatus {
    pub readonly: bool,
}

#[derive(Debug, Clone)]
pub enum ExportStatus {
    Global,
    Local,
    Normal,
    Declared,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ElshLvl(pub i32);

#[derive(Debug)]
pub struct Variable {
    pub var_type: Type,
    pub var_status: VariableStatus,
    pub var_export_status: ExportStatus,
    pub var_lvl: ElshLvl,
}

#[derive(Debug)]
pub struct Variables {
    vars: HashMap<String, Variable>,
    shopts: HashMap<String, bool>,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::String(val) => write!(f, "{}", val.to_string()),
            Self::Integer(val) => write!(f, "{}", val.to_string()),
            Self::Float(val) => write!(f, "{}", val.to_string()),
            Self::Array(val) => {
                let array_str_vec: Vec<String> =
                    val.iter().map(|x| x.to_string()).collect::<Vec<String>>();
                write!(f, "{:?}", &array_str_vec)
            }
        }
    }
}

impl Add for Type {
    type Output = Self;

    fn add(self, to_add: Self) -> Self {
        match self {
            Self::String(val) => Type::String(val + &to_add.to_string()),
            Self::Integer(val) => match to_add {
                Type::Integer(gotten_to_add) => Type::Integer(val + gotten_to_add),
                _ => unreachable!("Can't get an int and a nonint"),
            },
            Self::Float(val) => match to_add {
                Type::Float(gotten_to_add) => Type::Float(val + gotten_to_add),
                _ => unreachable!("Can't get a float and a nonfloat"),
            },
            Self::Array(val) => {
                let mut add_array: Vec<Type> = vec![];
                add_array.push(Type::Array(val));
                add_array.push(to_add);
                Type::Array(add_array)
            }
        }
    }
}

impl Variables {
    pub fn new() -> Self {
        let mut setup = Variables {
            vars: HashMap::new(),
            shopts: HashMap::new(),
        };
        setup.set(
            "ELSH_VERSION",
            Variable {
                // Set elsh version based on Cargo.toml version
                var_type: Type::String(env!("CARGO_PKG_VERSION").to_string()),
                var_status: VariableStatus { readonly: true },
                var_export_status: ExportStatus::Global,
                var_lvl: ElshLvl(0),
            },
        );
        setup.set(
            "PATH",
            Variable {
                var_type: Type::Array(vec![
                    Type::String("/usr/local/sbin".to_string()),
                    Type::String("/usr/local/bin".to_string()),
                    Type::String("/usr/sbin".to_string()),
                    Type::String("/usr/bin".to_string()),
                    Type::String("/bin".to_string()),
                    Type::String("/sbin".to_string()),
                    Type::String("/usr/games".to_string()),
                    Type::String("/usr/local/games".to_string()),
                ]),
                var_status: VariableStatus { readonly: false },
                var_export_status: ExportStatus::Global,
                var_lvl: ElshLvl(0),
            },
        );
        setup
    }

    pub fn set(&mut self, key: &str, value: Variable) {
        if self.vars.contains_key(&key.to_string()) {
            *self.vars.get_mut(&key.to_string()).unwrap() = value;
        } else {
            self.vars.insert(key.to_string(), value);
        }
    }

    pub fn set_shopt(&mut self, key: &str, value: bool) {
        if self.shopts.contains_key(&key.to_string()) {
            *self.shopts.get_mut(&key.to_string()).unwrap() = value;
        } else {
            self.shopts.insert(key.to_string(), value);
        }
    }

    pub fn delete(&mut self, key: String) {
        self.vars.remove(&key);
    }

    pub fn get(&self, key: &str) -> Option<&Variable> {
        self.vars.get(key)
    }
}
