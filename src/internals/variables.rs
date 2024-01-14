use std::collections::HashMap;

#[derive(Debug)]
pub enum Type {
    String(String),
    Integer(i32),
    Float(f32),
    Array(Vec<Type>),
}

#[derive(Debug)]
pub struct Variables {
    /// First String is the variable name, second is the type
    vars: HashMap<String, Type>,
    shopts: HashMap<String, bool>,
}

impl Variables {
    pub fn new() -> Self {
        let mut setup = Variables {
            vars: HashMap::new(),
            shopts: HashMap::new(),
        };
        setup.set("ELSH_VERSION", Type::String("0.0.1".to_string()));
        setup
    }

    pub fn set(&mut self, key: &str, value: Type) {
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

    pub fn get(&mut self, key: &str) -> Option<&Type> {
        self.vars.get(key)
    }
}
