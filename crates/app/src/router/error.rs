#[derive(Debug)]
pub enum RouterError {
    UnknownCommand(String),
    ExecutionFailed(String),
}