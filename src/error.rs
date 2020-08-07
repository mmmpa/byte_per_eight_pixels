#[derive(Debug, Eq, PartialEq)]
pub enum EightPxUintEightError {
    InvalidLengthData,
}

#[cfg(feature = "std")]
impl std::fmt::Display for EightPxUintEightError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for EightPxUintEightError {}
