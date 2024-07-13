pub trait Completer {
    fn complete(&self, input: &str) -> String;
}
