#[derive(Debug, Eq, PartialEq)]
pub enum BytePerEightPixelsError {
    SomethingWrong(String),
    InvalidLengthData,
}

impl std::fmt::Display for BytePerEightPixelsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for BytePerEightPixelsError {}
