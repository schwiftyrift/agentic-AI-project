use serde_json::{json, Value};

const MODEL : &str = "qwen3.8-27b-turbo-fable-cold-fusion-735-882-heretic-uncensored-neo-coder-max-mtp"; 
pub async fn chat(
    client: &reqwest::Client,
    url: &str,
    messages: &[Value],
    tools: &Value,
) -> Result<Value, Box<dyn std::error::Error>> {
    let payload = json!({"model": MODEL, "messages": messages, "tools": tools});
        let res = client
            .post(url)
            .json(&payload)
            .send()
            .await?;

        let body: Value = res.json().await?;
        let msg = body["choices"][0]["message"].clone();
        Ok(msg)
}