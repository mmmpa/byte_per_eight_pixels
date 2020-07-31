#[derive(Debug, Eq, PartialEq)]
pub enum EightPxUintEightError {
    SomethingWrong(String),
    InvalidLengthData,
}

impl std::fmt::Display for EightPxUintEightError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for EightPxUintEightError {}
