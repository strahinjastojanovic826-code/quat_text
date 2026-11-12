use crate::quat::Quat;
use crate::error::QuatError;

impl Quat {
    /// Converts a DNA/RNA nucleotide character to a Quat (A=00, C=01, G=10, T/U=11)
    pub fn from_dna(c: char) -> Result<Self, QuatError> {
        match c.to_ascii_uppercase() {
            'A' => Ok(Quat::V00),
            'C' => Ok(Quat::V01),
            'G' => Ok(Quat::V10),
            'T' | 'U' => Ok(Quat::V11),
            _ => Err(QuatError::InvalidCharacter(c)),
        }
    }

    /// Converts a Quat back to its DNA/RNA nucleotide character
    pub fn to_dna(self) -> char {
        match self {
            Quat::V00 => 'A',
            Quat::V01 => 'C',
            Quat::V10 => 'G',
            Quat::V11 => 'T',
        }
    }
}