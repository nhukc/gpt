extern crate gpt;

use gpt::completer::{Completer, GptCompleter};

#[tokio::main]
async fn main() {
    // Initialize the GPT Completer
    let gpt_completer = GptCompleter;

    // Generate a completion for the prompt "Hello, my name is"
    let response = gpt_completer.complete("AI Chatbot: Hello, my name is ").await;
    println!("{}", response.expect("Hopefully this passed"));
}

