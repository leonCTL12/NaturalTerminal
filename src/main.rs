mod large_language_model_interface;
mod prompt_preprocessor;
mod tool_interface;
mod utilities;

use crate::large_language_model_interface::configs::config::LLMProvider;
use crate::large_language_model_interface::configs::configs_handler;
use crate::large_language_model_interface::large_language_model_trait::NaturalLanguageProcessor;
use crate::large_language_model_interface::ollama_llm::ollama_llm::OllamaLLM;
use crate::large_language_model_interface::open_router_llm::open_router_llm::OpenRouterLLM;
use crate::prompt_preprocessor::preprocess_prompt;
use crate::tool_interface::cli::Cli;
use crate::tool_interface::commands::Commands;
use crate::utilities::secret_manager;
use anyhow::Result;
use clap::Parser;
use clipboard::{ClipboardContext, ClipboardProvider};
use nano_vectordb_rs::{Data, NanoVectorDB, constants};
use serde_json::json;
use std::io::{self, Write};
use tempfile::NamedTempFile;

#[tokio::main]
async fn main() -> io::Result<()> {
    test_nano_db();
    let cli = Cli::parse();

    match cli.command {
        Commands::Natural => {
            let config = match configs_handler::get_config() {
                Ok(config) => config,
                Err(e) => {
                    println!("Error getting config: {}", e);
                    println!("Please run `naturalterminal init` to set up the tool");
                    return Ok(());
                }
            };
            print!("What do you want terminal to do in natural language? ");
            io::stdout().flush()?; // Ensure the prompt is displayed before reading input

            let mut input_string = String::new();
            io::stdin().read_line(&mut input_string)?;
            let input_string = input_string.trim(); // Remove any trailing newline
            let prompt = preprocess_prompt(input_string);

            match config.provider {
                LLMProvider::Ollama => {
                    let ollama_llm = OllamaLLM;
                    handle_natural_input(&ollama_llm, &prompt).await;
                }
                LLMProvider::OpenRouter => {
                    let open_router_llm = OpenRouterLLM;
                    handle_natural_input(&open_router_llm, &prompt).await;
                }
            }
        }
        Commands::SetProvider { provider } => {
            println!("Setting provider to: {}", provider);
            match set_provider(provider) {
                Ok(_) => println!("Provider set successfully"),
                Err(e) => println!("Error setting provider: {}", e),
            }
        }
        Commands::SetOllamaModel { model } => match configs_handler::set_ollama_model(&model) {
            Ok(_) => println!("Ollama model set successfully"),
            Err(e) => println!("Error setting Ollama model: {}", e),
        },
        Commands::SetOpenRouterAPIKey { api_key } => {
            match secret_manager::set_open_router_api_key(&api_key) {
                Ok(_) => println!("Open Router API key set successfully"),
                Err(e) => println!("Error setting Open Router API key: {}", e),
            }
        }
        Commands::SetOpenRouterModel { model } => {
            match configs_handler::set_open_router_model(&model) {
                Ok(_) => println!("Open Router model set successfully"),
                Err(e) => println!("Error setting Open Router model: {}", e),
            }
        }
        Commands::Init => match initial_set_up() {
            Ok(_) => println!("Initial setup completed successfully"),
            Err(e) => println!("Error during initial setup: {}", e),
        },
    }
    Ok(())
}

async fn handle_natural_input(processor: &impl NaturalLanguageProcessor, input: &str) {
    let result = match processor.process_natural_input(input).await {
        Ok(result) => result,
        Err(e) => {
            println!("Error processing input: {}", e);
            return;
        }
    };
    let mut ctx: ClipboardContext =
        ClipboardProvider::new().expect("Failed to initialize clipboard");
    ctx.set_contents(result.clone())
        .expect("Failed to set clipboard contents");
    println!("{}", result);
    println!("The command above is on your copy buffer, you can paste it in your terminal");
}

fn set_provider(provider: String) -> Result<(), Box<dyn std::error::Error>> {
    let provider = match provider.as_str() {
        "ollama" => LLMProvider::Ollama,
        "openrouter" => LLMProvider::OpenRouter,
        _ => return Err("Invalid provider".into()),
    };
    configs_handler::set_provider(provider)
}

fn initial_set_up() -> Result<(), Box<dyn std::error::Error>> {
    println!("Thanks for using Natural Terminal");
    println!("Which Provider do you want to use? (ollama/openrouter)");
    let mut provider = String::new();
    io::stdin()
        .read_line(&mut provider)
        .expect("Failed to read line");
    let provider = provider.trim().to_string();
    set_provider(provider.to_string())?;
    if provider == "ollama" {
        println!("Which Ollama model do you want to use? Enter for default (llama3.2)");
        let mut model = String::new();
        io::stdin()
            .read_line(&mut model)
            .expect("Failed to read line");
        let mut model = model.trim();
        if model == "" {
            model = "llama3.2";
        }
        configs_handler::set_ollama_model(model)
    } else if provider == "openrouter" {
        println!("Which Open Router model do you want to use?");
        let mut model = String::new();
        io::stdin()
            .read_line(&mut model)
            .expect("Failed to read line");
        let model = model.trim();
        configs_handler::set_open_router_model(model)?;
        println!("Please set your Open Router API key");
        let mut api_key = String::new();
        io::stdin()
            .read_line(&mut api_key)
            .expect("Failed to read line");
        let api_key = api_key.trim();
        secret_manager::set_open_router_api_key(api_key)
    } else {
        Err("Invalid provider".into())
    }
}

fn test_nano_db() -> Result<(), Box<dyn std::error::Error>> {
    // Create temporary storage file
    let temp_file = NamedTempFile::new()?;
    let db_path = temp_file.path().to_str().unwrap();
    println!("Using temp file: {}", temp_file.path().display());
    // Initialize database with 3-dimensional vectors
    let mut db = NanoVectorDB::new(3, db_path)?;

    // Create sample data with metadata
    let samples = vec![
        Data {
            id: "vec1".into(),
            vector: vec![1.02, 2.0, 3.0],
            fields: [("color".into(), json!("red"))].into(),
        },
        Data {
            id: "vec2".into(),
            vector: vec![-4.0, 5.0, 6.0],
            fields: [("color".into(), json!("blue"))].into(),
        },
        Data {
            id: "vec3".into(),
            vector: vec![7.0, 8.0, -9.0],
            fields: [("color".into(), json!("green"))].into(),
        },
    ];

    // Upsert data and show results
    let (updated, inserted) = db.upsert(samples)?;
    println!("Updated IDs: {:?}", updated);
    println!("Inserted IDs: {:?}\n", inserted);

    // Persist to disk
    db.save()?;

    // Query similar vectors
    let query_vec = vec![0.1, 0.2, 0.3]; // Should be closest to vec1
    let results = db.query(&query_vec, 1, None, None);

    println!("Top 1 result:");
    for result in results {
        println!(
            "- ID: {} | Color: {} | Score: {:.4}",
            result[constants::F_ID],
            result["color"],
            result[constants::F_METRICS]
        );
    }

    // Delete a vector
    db.delete(&["vec3".into()]);
    db.save()?;

    println!("\nAfter deletion:");
    println!("Total vectors: {}", db.len());

    Ok(())
}
