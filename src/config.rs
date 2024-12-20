pub const API_URL: &str = "http://ollama.local/api/generate";
pub const MODEL_NAME: &str = "llama3.2-vision";
pub const DEFAULT_PROMPT: &str = "
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
