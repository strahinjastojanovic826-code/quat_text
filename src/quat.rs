use crate::error::QuatError;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Quat {
    V00 = 0b00,
    V01 = 0b01,
    V10 = 0b10,
    V11 = 0b11,
}

impl TryFrom<u8> for Quat {
    type Error = QuatError;

    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val & 0b11 {
            0 => Ok(Quat::V00),
            1 => Ok(Quat::V01),
            2 => Ok(Quat::V10),
            3 => Ok(Quat::V11),
            _ => Err(QuatError::InvalidBitPattern(val)),
        }
    }
}

impl TryFrom<char> for Quat {
    type Error = QuatError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '0' => Ok(Quat::V00),
            '1' => Ok(Quat::V01),
            '2' => Ok(Quat::V10),
            '3' => Ok(Quat::V11),
            _ => Err(QuatError::InvalidCharacter(c)),
        }
    }
}

impl From<Quat> for char {
    fn from(q: Quat) -> Self {
        match q {
            Quat::V00 => '0',
            Quat::V01 => '1',
            Quat::V10 => '2',
            Quat::V11 => '3',
        }
    }
}

impl fmt::Display for Quat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}