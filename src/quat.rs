use crate::error::QuatError;
use std::fmt;
use std::ops::{BitAnd, BitOr, BitXor, Not};

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

impl BitAnd for Quat {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        let val = (self as u8) & (rhs as u8);
        // Safe because bitwise AND of 2-bit numbers always stays within 0..=3
        unsafe { std::mem::transmute(val) }
    }
}

impl BitOr for Quat {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        let val = (self as u8) | (rhs as u8);
        unsafe { std::mem::transmute(val) }
    }
}

impl BitXor for Quat {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        let val = (self as u8) ^ (rhs as u8);
        unsafe { std::mem::transmute(val) }
    }
}

impl Not for Quat {
    type Output = Self;
    fn not(self) -> Self::Output {
        // Quaternary complement (bitwise NOT masked to 2 bits)
        let val = (!(self as u8)) & 0b11;
        unsafe { std::mem::transmute(val) }
    }
}