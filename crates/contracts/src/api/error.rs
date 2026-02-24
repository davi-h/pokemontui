#[derive(Debug)]
pub enum ApiError {
    NotFound,
    Http (u16),
    Network (String),
    Parse (String),
    Timeout,
    Unknown
}