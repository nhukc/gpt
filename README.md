# GPT Completer

This is a simple GPT Completer that uses the OpenAI API to generate text completions.

## Usage

To use this completer, you will need to set the `OPENAI_API_KEY` environment variable to your OpenAI API key.

```bash
export OPENAI_API_KEY=your_api_key
```

Then, you can use the `GptCompleter` struct to generate text completions. Here is an example:

```rust
mod completer;

use completer::{Completer, GptCompleter};

fn main() {
    // Initialize the GPT Completer
    let gpt_completer = GptCompleter;

    // Generate a completion for the prompt "Hello, my name is"
    let response = gpt_completer.complete("AI Chatbot: Hello, my name is ");
    println!("{}", response);
}
```

This will print out a completion for the prompt, such as "AI Chatbot: Hello, my name is AI Chatbot. I am an artificial intelligence".
