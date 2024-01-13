use core::fmt;
use std::collections::HashMap;

#[derive(Debug)]
pub enum Type {
    String(String),
    Integer(i32),
    Float(f32),
}

#[derive(Debug)]
pub struct Variables {
    /// First String is the variable name, second is the type
    vars: HashMap<String, Type>,
}

impl Variables {
    pub fn new() -> Self {
        let mut setup = Variables { vars: HashMap::new() };
        setup.set("ELSH_VERSION", Type::String(String::from("0.0.1")));
        setup
    }

    pub fn set(&mut self, key: &str, value: Type) {
        if self.vars.contains_key(&key.to_string()) {
            *self.vars.get_mut(&key.to_string()).unwrap() = value;
        } else {
            self.vars.insert(key.to_string(), value);
        }
    }

    pub fn delete(&mut self, key: String) {
        self.vars.remove(&key);
    }

    pub fn get(&mut self, key: &str) -> Option<&Type> {
        self.vars.get(key)
    }
}
