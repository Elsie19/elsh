use core::fmt;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Type {
    String(String),
    Integer(i32),
    Float(f32),
    Array(Vec<Type>),
}

#[derive(Debug)]
pub struct VariableStatus {
    pub readonly: bool,
}

#[derive(Debug)]
pub enum ExportStatus {
    Global,
    Local,
    Normal,
    Declared,
}

#[derive(Debug)]
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
    /// First String is the variable name, second is the type
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

impl Variables {
    pub fn new() -> Self {
        let mut setup = Variables {
            vars: HashMap::new(),
            shopts: HashMap::new(),
        };
        setup.set(
            "ELSH_VERSION",
            Variable {
                var_type: Type::String("0.0.1".to_string()),
                var_status: VariableStatus { readonly: true },
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
