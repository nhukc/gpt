use derive_more::Display;

#[derive(Debug)]
#[derive(Display)]
pub enum CompleterError {
    #[display(fmt = "Completer error: connection failed")]
    ConnectionFailed,
    #[display(fmt = "Completer error: not supported")]
    NotSupported
}

pub struct CompletionResult {
    pub prompt: String,
    pub completion: String,
}

pub trait Completer {
    fn complete(&self, input: String) -> Result<CompletionResult, CompleterError>;
}

