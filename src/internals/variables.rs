use core::fmt;
use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Type {
    String(String),
    Integer(u32),
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

#[derive(Debug, PartialEq)]
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

#[derive(Debug)]
pub struct Commands {
    cmds: HashMap<String, PathBuf>,
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

impl Commands {
    pub fn new(vars: &Variables) -> Self {
        let mut setup = Commands {
            cmds: HashMap::new(),
        };
        let paths_from_vars = vars.get("PATH").expect("Something went very wrong and elsh was not initialized with `PATH` array!");
        let path_directories = match &paths_from_vars.var_type {
            Type::Array(values) => values.iter().map(|x| x.to_string()).collect::<Vec<String>>(),
            _ => unreachable!("We are inside PATH matching, so if we don't have an array, we fucked up big time."),
        };
        // Loop over every directory we have and collect the files and slap dash them into the
        // hashmap
        for directory in path_directories {
            for file in fs::read_dir(directory).unwrap() {
                setup.cmds.insert(file.as_ref().unwrap().file_name().into_string().unwrap(), file.unwrap().path());
            }
        }
        setup
    }

    pub fn get_path(&self, name: String) -> Option<&PathBuf> {
        self.cmds.get(&name)
    }
}
