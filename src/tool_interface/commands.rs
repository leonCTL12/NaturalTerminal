use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    /// Echo back the natural language command
    Natural,
    /// Set the provider for the large language model
    SetProvider {
        provider: String,
    },
    SetOllamaModel {
        model: String,
    },
    SetOpenRouterModel {
        model: String,
    },
    SetOpenRouterAPIKey {
        api_key: String,
    },
    Init,
}
