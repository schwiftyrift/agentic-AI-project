use serde_json::{json, Value};

pub fn chat(
    client: &reqwest::blocking::Client,
    url: &str,
    messages: &[Value],
    tools: &Value,
) -> Result<Value, Box<dyn std::error::Error>> {
    let payload = json!({"model": "qwen/qwen3.8-27b", "messages": messages, "tools": tools});
        let res = client
            .post(url)
            .json(&payload)
            .send()?;

        println!("Status: {}", res.status());
        let body: Value = res.json()?;
        let msg = body["choices"][0]["message"].clone();
        Ok(msg)
}