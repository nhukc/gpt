[![Rust](https://github.com/nhukc/gpt/actions/workflows/rust.yml/badge.svg)](https://github.com/nhukc/gpt/actions/workflows/rust.yml)

# GPT Completer

This is a simple GPT Completer that uses the OpenAI API to generate text completions.

## Usage

The crate can be added to your Cargo.toml with the following snippet.
```toml
[dependencies]
gpt = { git = "https://github.com/nhukc/gpt.git", branch = "main" }
```

To use this completer, you will need to set the `OPENAI_API_KEY` environment variable to your OpenAI API key.

```bash
export OPENAI_API_KEY=your_api_key
```

Then, you can use the `GptCompleter` struct to generate text completions.

## Example Code
The example code is located in the examples directory and demonstrates how to
use the GptCompleter struct to generate text completions.  Here is the example
code:

```rust
extern crate gpt;

use gpt::completer::{Completer, GptCompleter};

fn main() {
    // Initialize the GPT Completer
    let gpt_completer = GptCompleter;

    // Generate a completion for the prompt "Hello, my name is"
    let response = gpt_completer.complete("AI Chatbot: Hello, my name is ");
    println!("{}", response);
}
```

This will print out a completion for the prompt, such as "AI Chatbot: Hello, my name is AI Chatbot. I am an artificial intelligence".

## Running the Example
To run the example provided in this project, follow these steps:

Ensure that you have set the `OPENAI_API_KEY` environment variable as shown above.
Use the `cargo run --example basic` command to execute the example.
```bash
cargo run --example basic
```
