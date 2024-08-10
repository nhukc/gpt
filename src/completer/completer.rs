use derive_more::Display;

#[derive(Debug)]
#[derive(Display)]
pub enum CompleterError {
    #[display(fmt = "Completer error: connection failed")]
    ConnectionFailed,
    #[display(fmt = "Completer error: not supported")]
    NotSupported
}

pub trait Completer {
    fn complete(&self, input: String) -> Result<String, CompleterError>;
}
