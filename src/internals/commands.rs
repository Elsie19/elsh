use crate::internals::variables::{Type, Variables};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
// This is for PATH commands
pub struct Commands {
    cmds: HashMap<String, PathBuf>,
}

#[derive(Debug)]
// This is for a specific command structure
pub struct Command {
    command: String,
    args: Vec<String>,
}

impl Commands {
    pub fn new(vars: &Variables) -> Self {
        let mut setup = Commands {
            cmds: HashMap::new(),
        };
        let paths_from_vars = vars
            .get("PATH")
            .expect("Something went very wrong and elsh was not initialized with `PATH` array!");
        let path_directories = match &paths_from_vars.var_type {
            Type::Array(values) => values
                .iter()
                .map(|path| path.to_string())
                .collect::<Vec<String>>(),
            _ => unreachable!(
                "We are inside PATH matching, so if we don't have an array, we fucked up big time."
            ),
        };
        // Loop over every directory we have and collect the files and slap dash them into the
        // hashmap
        for directory in path_directories {
            for file in fs::read_dir(directory).unwrap() {
                setup.cmds.insert(
                    file.as_ref().unwrap().file_name().into_string().unwrap(),
                    file.unwrap().path(),
                );
            }
        }
        setup
    }

    pub fn get_path(&self, name: String) -> Option<&PathBuf> {
        self.cmds.get(&name)
    }
}
