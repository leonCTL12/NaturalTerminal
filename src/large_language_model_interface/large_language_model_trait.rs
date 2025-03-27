pub trait NaturalLanguageProcessor {
    async fn process_natural_input(
        &self,
        input: &str,
    ) -> Result<String, Box<dyn std::error::Error>>;
}
