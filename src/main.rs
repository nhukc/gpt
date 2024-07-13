mod completer;

use completer::{Completer, GptCompleter};

fn main() {
    // Initialize the GPT Completer
    let gpt_completer = GptCompleter;

    // Generate a completion for the prompt "Hello, my name is"
    let response = gpt_completer.complete("AI Chatbot: Hello, my name is ");
    println!("{}", response);
}
