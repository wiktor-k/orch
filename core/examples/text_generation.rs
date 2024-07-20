//! This example demonstrates how to use the `Executor` to generate a response from the LLM.
//! We construct an `Ollama` instance and use it to generate a response.

use orch::execution::*;
use orch::lm::*;

#[tokio::main]
async fn main() {
    // ! Change this to use a different provider.
    let provider = LanguageModelProvider::Ollama;

    let prompt = "What is 2+2?";

    println!("Prompt: {prompt}");
    println!("---");

    // Use a different language model, per the `provider` variable (feel free to change it).
    let open_ai_api_key = {
        if provider == LanguageModelProvider::OpenAi {
            std::env::var("OPENAI_API_KEY")
                .unwrap_or_else(|_| panic!("OPENAI_API_KEY environment variable not set"))
        } else {
            String::new()
        }
    };
    let lm: Box<dyn LanguageModel> = match provider {
        LanguageModelProvider::Ollama => Box::new(OllamaBuilder::new().try_build().unwrap()),
        LanguageModelProvider::OpenAi => Box::new(
            OpenAiBuilder::new()
                .with_api_key(&open_ai_api_key)
                .try_build()
                .unwrap(),
        ),
    };

    let executor = TextExecutorBuilder::new()
        .with_lm(&*lm)
        .try_build()
        .unwrap();
    let response = executor.execute(prompt).await.expect("Execution failed");

    println!("Response:");
    println!("{}", response.content);
}
