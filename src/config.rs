use dotenv::dotenv;
use std::env;

#[derive(Default)]
pub struct Config {
    pub completion_endpoint: String,
    pub completion_model: String,
    pub completion_prompt: String,
    pub embedding_endpoint: String,
    pub embedding_model: String,
}

impl Config {
    pub fn build() -> Result<Self, env::VarError> {
        const DEFAULT_COMPLETION_ENDPOINT: &str = "http://ollama.local/api/generate";
        const DEFAULT_COMPLETION_MODEL: &str = "llama3.2-vision";
        const DEFAULT_COMPLETION_PROMPT: &str = "
            Analyze this image in detail and provide a comprehensive description in the following format:

            Primary subject/focus (1-2 sentences describing the main subject)
            Visual characteristics:

            Colors (dominant and accent colors)
            Lighting conditions and atmosphere
            Composition and framing

            Contextual details:

            Setting/background
            Time of day/season (if applicable)
            Notable objects or elements

            Technical aspects:

            Image quality
            Perspective/angle
            Any distinctive photographic techniques

            Semantic tags: [list 5-7 key descriptive tags]

            Finally, provide a concise, information-dense prompt (2-3 sentences) that captures the most distinctive and searchable aspects of this image.
        ";
        const DEFAULT_EMBEDDING_ENDPOINT: &str = "http://ollama.local/api/embed";
        const DEFAULT_EMBEDDING_MODEL: &str = "nomic-embed-text";

        dotenv().ok();
        let completion_endpoint = env::var("COMPLETION_ENDPOINT")
            .unwrap_or_else(|_| DEFAULT_COMPLETION_ENDPOINT.to_string());
        let completion_model =
            env::var("COMPLETION_MODEL").unwrap_or_else(|_| DEFAULT_COMPLETION_MODEL.to_string());
        let completion_prompt =
            env::var("COMPLETION_PROMPT").unwrap_or_else(|_| DEFAULT_COMPLETION_PROMPT.to_string());
        let embedding_endpoint = env::var("EMBEDDING_ENDPOINT")
            .unwrap_or_else(|_| DEFAULT_EMBEDDING_ENDPOINT.to_string());
        let embedding_model =
            env::var("EMBEDDING_MODEL").unwrap_or_else(|_| DEFAULT_EMBEDDING_MODEL.to_string());

        Ok(Self {
            completion_endpoint,
            completion_model,
            completion_prompt,
            embedding_endpoint,
            embedding_model,
        })
    }
}
