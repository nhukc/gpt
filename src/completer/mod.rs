mod completer;
mod gpt_completer;
mod hardcoded_completer;
mod tree;

pub use completer::Completer;
pub use gpt_completer::GptCompleter;

use crate::completer::completer::CompleterError::NotSupported;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::completer::gpt_completer::GptCompleter;
    use crate::completer::hardcoded_completer::HardcodedCompleter;
    use crate::completer::{tree::characteristics, tree::parts};

    #[tokio::test]
    async fn test_tree_parts() {
        let hardcoded_completer = HardcodedCompleter;
        let gpt_completer = GptCompleter;

        let (input, expected_output) = tree::parts();
        assert_eq!(hardcoded_completer.complete(input).await.unwrap(), expected_output);
        assert_eq!(gpt_completer.complete(input).await.unwrap(), expected_output);
    }

    #[tokio::test]
    async fn test_tree_characteristics() {
        let hardcoded_completer = HardcodedCompleter;
        let gpt_completer = GptCompleter;

        let (input, expected_output) = tree::characteristics();
        assert_eq!(hardcoded_completer.complete(input).await.unwrap(), expected_output);
        assert_eq!(gpt_completer.complete(input).await.unwrap(), expected_output);
    }

    #[tokio::test]
    async fn test_unmatched() {
        let hardcoded_completer = HardcodedCompleter;
        let _gpt_completer = GptCompleter; // TODO: Figure out how to test this.

        let unmatched_input = "Some unmatched input";
        match hardcoded_completer.complete(unmatched_input).await {
            Err(NotSupported) => {},
            Ok(_) | Err(_) => { panic!("How did this pass?"); }
        }
    }
}
