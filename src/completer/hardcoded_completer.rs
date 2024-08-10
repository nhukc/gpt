use crate::completer::Completer;

pub struct HardcodedCompleter;

use crate::completer::completer::CompleterError;
use crate::completer::completer::CompleterError::NotSupported;

impl HardcodedCompleter {
    pub fn get_mappings() -> Vec<(&'static str, &'static str)> {
        vec![
            crate::completer::tree::parts(),
            crate::completer::tree::characteristics(),
            // Additional mappings can be added here
        ]
    }
}

impl Completer for HardcodedCompleter {
    fn complete(&self, input: String) -> Result<String, CompleterError> {
        let mappings = HardcodedCompleter::get_mappings();

        for (key, value) in mappings {
            if input.trim() == key.trim() {
                return Ok(value.trim().to_string());
            }
        }

        Err(NotSupported)
    }
}
