use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub provider: LLMProvider,
    pub open_router_model: String,
    pub ollama_model: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum LLMProvider{
    OpenRouter, Ollama
}