use std::collections::HashMap;

pub struct Context {
    pub args: Vec<String>,
    pub flags: HashMap<String, String>,
}

impl Context {
    pub fn new(args: Vec<String>) -> Self {
        Self {
            args,
            flags: HashMap::new(),
        }
    }
}