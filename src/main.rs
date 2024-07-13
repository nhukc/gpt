mod completer;

use completer::{Completer, GptCompleter};
use indoc::indoc;

pub const FIRST_CHARACTERISTIC: &str =
    indoc! {"The first adjective I think of when I hear '{object}' is: "};
pub const CHARACTERISTIC_LIST: &str = indoc! {"
    Adjectives describing an average {object} are:
    - {char_1}
    - "};

fn main() {
    // Initialize the GPT Completer
    let gpt_completer = GptCompleter;

    let object = "tree branch";

    // Define the series of prompts
    let prompt = FIRST_CHARACTERISTIC.replace("{object}", object);
    let response = gpt_completer.complete(&prompt);
    let first_characteristic = response.lines().nth(1).expect("No adjective");
    println!("{}", first_characteristic);

    let prompt = CHARACTERISTIC_LIST
        .replace("{object}", object)
        .replace("{char_1}", first_characteristic);
    let response = gpt_completer.complete(&prompt);
    let characteristics: Vec<_> = response.lines().skip(1).map(|s| s.replace("- ", "")).collect();
    println!("{:?}", characteristics);
}

// Function to parse the first word adjective from the response
fn parse_first_word(response: &str) -> Option<&str> {
    response.split_whitespace().next()
}
