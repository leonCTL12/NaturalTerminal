use crate::large_language_model_interface::configs::configs_handler;
use crate::large_language_model_interface::large_language_model_trait::NaturalLanguageProcessor;
use serde_json::{Value, json};
use std::error::Error;
use std::time::Duration;

pub struct OllamaLLM;

impl NaturalLanguageProcessor for OllamaLLM {
    async fn process_natural_input(&self, input: &str) -> Result<String, Box<dyn Error>> {
        println!("Ollama LLM Processing......");
        OllamaLLM::send_to_llm(input).await
    }
}

impl OllamaLLM {
    async fn send_to_llm(input: &str) -> Result<String, Box<dyn Error>> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(15)) // 15-second timeout
            .build()?;
        let url = "http://localhost:11434/api/generate";
        let config = match configs_handler::get_config() {
            Ok(config) => config,
            Err(e) => {
                println!("Error getting config: {}", e);
                return Err("Error getting config".into());
            }
        };
        let request_body = json!({
            "model": config.ollama_model,
            "prompt": input,
            "stream": false
        });

        let response = match client
            .post(url)
            .json(&request_body)
            .send()
            .await?
            .json::<Value>()
            .await
        {
            Ok(response) => response,
            Err(e) => {
                println!("Error getting response: {}", e);
                return Err(
                    "Make sure Ollama is running properly on your machine, Error getting response"
                        .into(),
                );
            }
        };

        let output = response["response"].as_str().unwrap_or("No response");
        Ok(output.to_string())
    }
}
