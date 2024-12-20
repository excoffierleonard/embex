use dotenv::dotenv;
use std::env;

const DEFAULT_API_URL: &str = "http://ollama.local/api/generate";
const DEFAULT_MODEL_NAME: &str = "llama3.2-vision";
const DEFAULT_PROMPT: &str = "
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

pub struct Config {
    pub api_url: String,
    pub model_name: String,
    pub prompt: String,
}

impl Config {
    pub fn build() -> Result<Self, env::VarError> {
        dotenv().ok();
        let api_url = env::var("API_URL").unwrap_or_else(|_| DEFAULT_API_URL.to_string());
        let model_name = env::var("MODEL_NAME").unwrap_or_else(|_| DEFAULT_MODEL_NAME.to_string());
        let prompt = env::var("PROMPT").unwrap_or_else(|_| DEFAULT_PROMPT.to_string());

        Ok(Self {
            api_url,
            model_name,
            prompt,
        })
    }
}
