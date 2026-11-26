use serde_json::{json, Value};
use std::process::Command;
use std::io::{self, Write};

use super::Tool;

pub struct RunShell;

impl Tool for RunShell {
    fn name(&self) -> &str {
        "run_shell"
    }

    fn definition(&self) -> Value {
        let description = format!(
            "Runs a command in a shell. The operating system is {}.",
            std::env::consts::OS
        );

        json!({"type": "function",
            "function": {"name": "run_shell",
            "description": description,
            "parameters": {
                "type": "object",
                "properties": {
                    "cmd": {
                        "type": "string",
                        "description": "Command to run in the shell, for example 'ls' or 'ls -la'"
                    }
                },
            "required": ["cmd"]
            }
                    }
                    })
    }

    fn run(&self, args: &Value) -> String {
            let cmd = args["cmd"].as_str().unwrap_or("");
            run_shell(cmd)
    }
}

pub fn run_shell(cmd: &str) -> String {
    // detect the operating system it is running on
    let (shell, flag) = if cfg!(target_os = "windows") {
        ("cmd", "/C")
    } else {
        ("sh", "-c")
    };
    // run the command and then return the output
    println!("Model wants to run: {cmd}");
    print!("Run this? [y/N] ");
    let _ = io::stdout().flush();

    let mut confirm = String::new();

    io::stdin()
        .read_line(&mut confirm)
        .expect("Failed to read line");

    if confirm.trim().eq_ignore_ascii_case("y") {
        match Command::new(shell).arg(flag).arg(cmd).output() {
            Ok(o) => {
                let mut s = String::from_utf8_lossy(&o.stdout).into_owned();
                s.push_str(&String::from_utf8_lossy(&o.stderr));
                s.chars().take(4000).collect()
            }
            Err(e) => format!("error: {e}"),
        }
       
    } else {
        "user denied command".to_string()
    }
}