mod tools;
mod llm;
mod agent;


use serde_json::{json, Value};
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "http://192.168.1.147:1234/v1/chat/completions";
    let client = reqwest::Client::new();
    let mut messages: Vec<Value> = Vec::new();

    loop{
        let mut message = String::new();

        print!("> ");
        io::stdout().flush()?;
        io::stdin()
            .read_line(&mut message)
            .expect("Failed to read line");
        
        let input = message.trim();
        if input.is_empty() {
            continue;
        }
        if input == "exit" || input == "quit" {
            break;
        }
        messages.push(json!({"role": "user", "content": input}));
        
        agent::run_turn(&client, url, &mut messages).await?;
    };
    Ok(())
}
