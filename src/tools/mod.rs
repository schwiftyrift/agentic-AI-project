use std::collections::HashMap;
use serde_json::Value;

mod shell;
mod readfile;

pub use shell::RunShell;
pub use readfile::ReadFile;

pub struct Registry {
    tools: HashMap<String, Box<dyn Tool>>,
}

impl Registry {
    pub fn new() -> Self {
        Registry { tools: HashMap::new() }
    }

    pub fn register(&mut self, tool: Box<dyn Tool>) {
        self.tools.insert(tool.name().to_string(), tool);
    }

    pub fn definitions(&self) -> Value {
        let defs: Vec<Value> = self.tools.values().map(|t| t.definition()).collect();
        Value::Array(defs)
    }

    pub fn dispatch(&self, name: &str, args: &Value) -> String {
        match self.tools.get(name){
            Some(tool) => tool.run(args),
            None => format!("error: unknown tool '{name}'")
        }
    }
}

pub trait Tool {
    fn name(&self) -> &str;

    fn definition(&self) -> Value;

    fn run(&self, args: &Value) -> String;
}