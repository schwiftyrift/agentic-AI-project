use serde_json::{json, Value};
use crate::{llm, tools};

pub async fn run_turn(
    client: &reqwest::Client,
    url: &str,
    messages: &mut Vec<Value>,
) -> Result<(), Box<dyn std::error::Error>> {
    
    let mut registry = tools::Registry::new();
    registry.register(Box::new(tools::RunShell));
    registry.register(Box::new(tools::ReadFile));
    let tool_defs = registry.definitions();

    for _ in 0..10 {
        let msg = llm::chat(&client, url, messages, &tool_defs).await?;
        messages.push(msg.clone());

        match msg["tool_calls"].as_array() {
            Some(calls) if !calls.is_empty() => {
                for call in calls {
                    let name = call["function"]["name"].as_str().unwrap_or("");
                    let raw_args = call["function"]["arguments"].as_str().unwrap_or("{}");
                    let id = call["id"].as_str().unwrap_or("");

                    let args: Value = serde_json::from_str(raw_args)?;

                    let result = registry.dispatch(name, &args);

                    messages.push(json!({
                        "role": "tool",
                        "tool_call_id": id,
                        "content": result
                    }));
                }
            }
            _ => {
                println!("{}", msg["content"].as_str().unwrap_or(""));
                return Ok(());
            }
        }
    }
    Ok(())    
}
    
