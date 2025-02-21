use serde::{Deserialize, Serialize};

use crate::ollama_model;

/// Response from the Ollama API for obtaining information about local models.
/// Referenced from the Ollama API documentation [here](https://github.com/ollama/ollama/blob/fedf71635ec77644f8477a86c6155217d9213a11/docs/api.md#list-running-models).
#[derive(Debug, Serialize, Deserialize)]
pub struct OllamaApiModelsMetadata {
    pub models: Vec<OllamaApiModelMetadata>,
}

/// Response item from the Ollama API for obtaining information about local models.
///
/// Referenced from the Ollama API documentation [here](https://github.com/ollama/ollama/blob/fedf71635ec77644f8477a86c6155217d9213a11/docs/api.md#response-22).
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct OllamaApiModelMetadata {
    /// The name of the model (e.g., "mistral:latest")
    pub name: String,

    /// The Ollama identifier of the model (e.g., "mistral:latest")
    pub model: String,

    /// Size of the model in bytes
    pub size: usize,

    /// Digest of the model using SHA256 (e.g., "2ae6f6dd7a3dd734790bbbf58b8909a606e0e7e97e94b7604e0aa7ae4490e6d8")
    pub digest: String,

    /// Model expiry time in ISO 8601 format (e.g., "2024-06-04T14:38:31.83753-07:00")
    pub expires_at: Option<String>,

    /// More details about the model
    pub details: OllamaApiModelDetails,
}

/// Details about a running model in the API for listing running models (`GET /api/ps`).
///
/// Referenced from the Ollama API documentation [here](https://github.com/ollama/ollama/blob/fedf71635ec77644f8477a86c6155217d9213a11/docs/api.md#response-22).
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct OllamaApiModelDetails {
    /// Model identifier that this model is based on
    pub parent_model: String,

    /// Format that this model is stored in (e.g., "gguf")
    pub format: String,

    /// Model family (e.g., "ollama")
    pub family: String,

    /// Parameters of the model (e.g., "7.2B")
    pub parameter_size: String,

    /// Quantization level of the model (e.g., "Q4_0" for 4-bit quantization)
    pub quantization_level: String,
}

/// Request for generating a response from the Ollama API.
///
/// Referenced from the Ollama API documentation [here](https://github.com/ollama/ollama/blob/fedf71635ec77644f8477a86c6155217d9213a11/docs/api.md#generate-a-completion).
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct OllamaGenerateRequest {
    /// Model identifier (e.g., "mistral:latest")
    pub model: String,

    /// The prompt to generate a response for (e.g., "List all Kubernetes pods")
    pub prompt: String,

    /// The context parameter returned from a previous request to /generate, this can be used to keep a short conversational memory
    pub context: Option<Vec<i64>>,

    /// Optional list of base64-encoded images (for multimodal models such as `llava`)
    pub images: Option<Vec<String>>,

    /// Optional format to use for the response (currently only "json" is supported)
    pub format: Option<String>,

    /// Optional flag that controls whether the response is streamed or not (defaults to true).
    /// If `false`` the response will be returned as a single response object, rather than a stream of objects
    pub stream: Option<bool>,

    // System message (overrides what is defined in the Modelfile)
    pub system: Option<String>,

    /// Controls how long the model will stay loaded into memory following the request (default: 5m)
    pub keep_alive: Option<String>,
}

impl Default for OllamaGenerateRequest {
    fn default() -> Self {
        Self {
            model: ollama_model::CODESTRAL.to_string(),
            prompt: "".to_string(),
            stream: Some(false),
            format: None,
            images: None,
            system: Some("You are a helpful assistant".to_string()),
            keep_alive: Some("5m".to_string()),
            context: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct OllamaGenerateResponse {
    /// Model identifier (e.g., "mistral:latest")
    pub model: String,

    /// Time at which the response was generated (ISO 8601 format)
    pub created_at: String,

    /// The response to the prompt
    pub response: String,

    /// The encoding of the conversation used in this response, this can be sent in the next request to keep a conversational memory
    pub context: Option<Vec<i64>>,

    /// The duration of the response in nanoseconds
    pub total_duration: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OllamaGenerateStreamItemResponse {
    /// Model identifier (e.g., "mistral:latest")
    pub model: String,

    /// Time at which the response was generated (ISO 8601 format)
    pub created_at: String,

    /// The response to the prompt
    pub response: String,
}

/// Request for generating an embedding from the Ollama API.
/// Referenced from the Ollama API documentation [here](https://github.com/ollama/ollama/blob/fedf71635ec77644f8477a86c6155217d9213a11/docs/api.md#generate-embeddings).
///
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct OllamaEmbeddingsRequest {
    /// The string to generate an embedding for.
    pub prompt: String,

    /// The model to use for the embedding generation.
    pub model: String,
}

/// Response from the Ollama API for generating an embedding.
/// Referenced from the Ollama API documentation [here](https://github.com/ollama/ollama/blob/fedf71635ec77644f8477a86c6155217d9213a11/docs/api.md#generate-embeddings).
///
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct OllamaEmbeddingsResponse {
    /// The embedding for the prompt.
    pub embedding: Vec<f32>,
}
