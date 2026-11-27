use crate::quat::Quat;
use crate::error::QuatError;
use crate::QuatText;

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

    /// Converts a Quat to its RNA nucleotide character (`'A'`, `'C'`, `'G'`, `'U'`).
#[must_use]
pub fn to_rna(self) -> char {
    match self {
        Quat::V00 => 'A',
        Quat::V01 => 'C',
        Quat::V10 => 'G',
        Quat::V11 => 'U',
    }
}

/// Returns the Watson-Crick complementary quat (`A<->T`, `C<->G`).
#[must_use]
pub fn complement(self) -> Self {
    !self
}

}

pub trait QuatBioExt {
    fn complement_in_place(&mut self);
    fn to_complement(&self) -> QuatText;
    fn to_reverse_complement(&self) -> QuatText;
    fn find_pattern(&self, pattern: &QuatText) -> Option<usize>;
    fn count_subsequence(&self, pattern: &QuatText) -> usize;
}

impl QuatBioExt for QuatText {
    fn complement_in_place(&mut self) {
        let full_bytes = self.len() / 4;
        for b in &mut self.data[..full_bytes] {
            *b = !*b;
        }
        let rem = self.len() % 4;
        if rem > 0 {
            let mask = !0u8 << ((4 - rem) * 2);
            self.data[full_bytes] ^= mask;
        }
    }

    fn to_complement(&self) -> QuatText {
        let mut copy = self.clone();
        copy.complement_in_place();
        copy
    }

    fn to_reverse_complement(&self) -> QuatText {
        let mut out = QuatText::with_capacity(self.len());
        for i in (0..self.len()).rev() {
            if let Ok(q) = self.get(i) {
                out.push(!q);
            }
        }
        out
    }

    fn find_pattern(&self, pattern: &QuatText) -> Option<usize> {
        if pattern.is_empty() {
            return Some(0);
        }
        if pattern.len() > self.len() {
            return None;
        }
        let limit = self.len() - pattern.len();
        'outer: for start in 0..=limit {
            for offset in 0..pattern.len() {
                if self.get(start + offset).ok() != pattern.get(offset).ok() {
                    continue 'outer;
                }
            }
            return Some(start);
        }
        None
    }

    fn count_subsequence(&self, pattern: &QuatText) -> usize {
        if pattern.is_empty() || pattern.len() > self.len() {
            return 0;
        }
        let mut count = 0;
        let mut i = 0;
        let limit = self.len() - pattern.len();
        while i <= limit {
            let mut matched = true;
            for offset in 0..pattern.len() {
                if self.get(i + offset).ok() != pattern.get(offset).ok() {
                    matched = false;
                    break;
                }
            }
            if matched {
                count += 1;
                i += pattern.len(); // non-overlapping count; use `i += 1` for overlapping
            } else {
                i += 1;
            }
        }
        count
    }
}