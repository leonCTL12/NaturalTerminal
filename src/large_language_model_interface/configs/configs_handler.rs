use std::fs;
use std::path::PathBuf;
use dirs::config_dir;
use crate::large_language_model_interface::configs::config::{Config, LLMProvider};
use crate::large_language_model_interface::configs::config::LLMProvider::Ollama;

fn get_config_path() -> PathBuf {
    let mut path = config_dir().expect("Could not find config directory");
    path.push("naturalterminal");
    path.push("config.json");
    path
}

fn config_exists() -> bool {
    let path = get_config_path();
    path.exists()
}

fn create_default_config() -> Result<(), Box<dyn std::error::Error>> {
    let path = get_config_path();

    let default_config = Config{
        provider: Ollama,
        open_router_model: "".to_string(),
        ollama_model: "llama3.2".to_string()
    };

    if let Some(parent) = path.parent() { 
        fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_string_pretty(&default_config)?;
    fs::write(&path, json)?;
    println!("Config saved to: {:?}", path);
    Ok(())
}

pub fn set_ollama_model(model: &str) -> Result<(), Box<dyn std::error::Error>> {
    if !config_exists() {
        create_default_config()?;
    }
    let path = get_config_path();
    let mut config: Config = serde_json::from_str(&fs::read_to_string(&path)?)?;
    config.ollama_model = model.to_string();
    let json = serde_json::to_string_pretty(&config)?;
    fs::write(&path, json)?;
    Ok(())
}

pub fn set_open_router_model(model: &str) -> Result<(), Box<dyn std::error::Error>> {
    if !config_exists() {
        create_default_config()?;
    }
    let path = get_config_path();
    let mut config: Config = serde_json::from_str(&fs::read_to_string(&path)?)?;
    config.open_router_model = model.to_string();
    let json = serde_json::to_string_pretty(&config)?;
    fs::write(&path, json)?;
    Ok(())
}

pub fn set_provider(provider: LLMProvider) -> Result<(), Box<dyn std::error::Error>> {
    if !config_exists() {
        create_default_config()?;
    }
    let path = get_config_path();
    let mut config: Config = serde_json::from_str(&fs::read_to_string(&path)?)?;
    config.provider = provider;
    let json = serde_json::to_string_pretty(&config)?;
    fs::write(&path, json)?;
    Ok(())
}

pub fn get_config() -> Result<Config, Box<dyn std::error::Error>> {
    if !config_exists() {
        Err("Config file does not exist")?;
    }
    let path = get_config_path();
    let config: Config = serde_json::from_str(&fs::read_to_string(&path)?)?;
    Ok(config)
}