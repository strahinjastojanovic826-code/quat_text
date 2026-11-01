use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QuatError {
    InvalidBitPattern(u8),
    InvalidCharacter(char),
    IndexOutOfBounds { index: usize, len: usize },
}

impl fmt::Display for QuatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QuatError::InvalidBitPattern(val) => {
                write!(f, "Invalid bit pattern for Quat: {:#b}", val)
            }
            QuatError::InvalidCharacter(c) => {
                write!(f, "Invalid character for Quat representation: '{}'", c)
            }
            QuatError::IndexOutOfBounds { index, len } => {
                write!(
                    f,
                    "Index out of bounds: index {} is out of range for length {}",
                    index, len
                )
            }
        }
    }
}

impl std::error::Error for QuatError {}