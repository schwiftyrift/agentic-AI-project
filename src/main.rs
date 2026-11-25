cduse serde_json::{json, Value};
use std::process::Command;

fn tool_definitions() -> Value {
    let description = format!("Runs a command in a shell. The operating system is {}.", std::env::consts::OS);
    json!([{"type": "function",
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
}])
}

fn run_shell(cmd: &str) -> String {
    // detect the operating system it is running on
    let (shell, flag) = if cfg!(target_os = "windows") {
        ("cmd", "/C")
    } else {
        ("sh", "-c")
    };
    // run the command and then return the output
    match Command::new(shell).arg(flag).arg(cmd).output() {
        Ok(o) => {
            let mut s = String::from_utf8_lossy(&o.stdout).into_owned();
            s.push_str(&String::from_utf8_lossy(&o.stderr));
            s.chars().take(4000).collect()
        }
        Err(e) => format!("error: {e}"),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "http://192.168.1.147:1234/v1/chat/completions";
    let mut messages: Vec<Value> = vec![
        json!({"role": "user", "content": "Tell me which files are in my current directory"}),
    ];

    let payload = json!({"model": "qwen/qwen3.8-27b", "messages": messages, "tools": tool_definitions()});
    let client = reqwest::blocking::Client::new();
    let res = client
        .post(url)
        .json(&payload)
        .send()?;

    println!("Status: {}", res.status());
    let body: Value = res.json()?;
    let reply = &body["choices"][0]["message"]["content"];
    //println!("{}", reply.as_str().unwrap());
    println!("{:#}", body);
    messages.push(body["choices"][0]["message"].clone());

    println!("History length: {}", messages.len());
    Ok(())
}
