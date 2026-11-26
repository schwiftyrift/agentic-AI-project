use serde_json::{json, Value};

use super::Tool;

pub struct ReadFile;

impl Tool for ReadFile {
    fn name(&self) -> &str { "read_file" }

    fn definition(&self) -> Value {
        let description = format!(
            "Read the raw contents of a file. The operating system is {}.",
            std::env::consts::OS
        );

        json!({"type": "function",
            "function": {"name": "read_file",
            "description": description,
            "parameters": {
                "type": "object",
                "properties": {
                    "path": {
                        "type": "string",
                        "description": "Path of the file to read"
                    }
                },
            "required": ["path"]
            }
                    }
                    })
    }

    fn run(&self, args: &Value) -> String {
        let path = args["path"].as_str().unwrap_or("");
        read_file(path)
    }
}

pub fn read_file(path: &str) -> String {
    match std::fs::read_to_string(path) {
        Ok(contents) => contents.chars().take(4000).collect(),
        Err(e) => format!("error: {e}"),
    }
}