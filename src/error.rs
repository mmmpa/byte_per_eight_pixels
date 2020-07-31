#[derive(Debug, Eq, PartialEq)]
pub enum EightPxU8Error {
    SomethingWrong(String),
    InvalidLengthData,
}

impl std::fmt::Display for EightPxU8Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for EightPxU8Error {}
