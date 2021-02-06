pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug)]
pub struct Error(String);

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
