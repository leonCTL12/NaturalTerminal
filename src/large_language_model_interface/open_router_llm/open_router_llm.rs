use crate::large_language_model_interface::configs::configs_handler;
use crate::large_language_model_interface::large_language_model_trait::NaturalLanguageProcessor;
use crate::large_language_model_interface::open_router_llm::data_structure::ChatCompletionResponse;
use crate::utilities::secret_manager;
use std::error::Error;
use std::time::Duration;

pub struct OpenRouterLLM;

impl NaturalLanguageProcessor for OpenRouterLLM {
    async fn process_natural_input(&self, input: &str) -> Result<String, Box<dyn Error>> {
        println!("Open Router LLM Processing......");
        OpenRouterLLM::send_to_llm(input).await
    }
}

impl OpenRouterLLM {
    async fn send_to_llm(input: &str) -> Result<String, Box<dyn Error>> {
        let url = "https://openrouter.ai/api/v1/chat/completions";
        let api_key = secret_manager::get_open_router_api_key()
            .expect("You have not set the Open Router API key");
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()?;
        let model = match configs_handler::get_config() {
            Ok(config) => config.open_router_model,
            Err(e) => {
                println!("Error getting config: {}", e);
                return Err("Error getting config".into());
            }
        };

        if model.is_empty() {
            return Err("Open Router model is not set".into());
        }

        let request_body = serde_json::json!({
            "model": model,
            "messages": [
                {
                    "role": "user",
                    "content": input
                }
            ]
        });

        let response = client
            .post(url)
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&request_body)
            .send()
            .await?;

        let response_json: ChatCompletionResponse = response.json().await?;
        let content = &response_json.choices[0].message.content;

        Ok(content.to_string())
    }
}
