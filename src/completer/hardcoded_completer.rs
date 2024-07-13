use crate::completer::Completer;

pub struct HardcodedCompleter;

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
    fn complete(&self, input: &str) -> String {
        let mappings = HardcodedCompleter::get_mappings();

        for (key, value) in mappings {
            if input.trim() == key.trim() {
                return value.trim().to_string();
            }
        }

        "OUTPUT NOT MATCHED".to_string()
    }
}
