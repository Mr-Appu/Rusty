use crate::models::general::llm::{AIResponse, Contents, Parts, Text, GenerationConfig};
use dotenv::dotenv;
use reqwest::Client;
use std::env;

// Call LLM (Gemini)
pub async fn call_gpt(text: Vec<Text>) -> Result<String, Box<dyn std::error::Error + Send>> {
    // Extract API Key information
    dotenv().ok();
    let api_key: String = env::var("API_KEY").expect("API_KEY not found in enviornment variables");

    // Gemini endpoint
    let url: String = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateContent?key={}",
        api_key
    );
    let url: &str = url.as_str();

    // Create Client
    let client: Client = Client::builder()
        .build()
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;

    // Create Parts && Contents
    let parts: Parts = Parts { parts: text };
    let generation_config:GenerationConfig = GenerationConfig {temperature: 0.4};
    let contents: Contents = Contents {
        contents: vec![parts],
        generationConfig: generation_config
    };

    // // Troubleshooting
    // let raw_res = client.post(url).json(&contents).send().await.unwrap();
    // println!("{}", raw_res.text().await.unwrap());

    // Extract API Response
    let res: AIResponse = client
        .post(url)
        .json(&contents)
        .send()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?
        .json()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;

    Ok(res.candidates[0].content.parts[0].text.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn tests_call_to_openai() {
        let text: Text = Text {
            text: "Hello Gemini".to_string(),
        };

        match call_gpt(vec![text]).await {
            Ok(res_str) => {
                dbg!(res_str);
                assert!(true);
            }
            Err(e) => {
                dbg!(e);
                assert!(false);
            }
        }
    }
}
