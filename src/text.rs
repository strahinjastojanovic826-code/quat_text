use crate::error::QuatError;
use crate::quat::Quat;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuatText {
    data: Vec<u8>,
    len: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QuatSlice<'a> {
    data: &'a [u8],
    len: usize,
}

impl QuatText {
    /// Creates a new, empty QuatText instance.
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            len: 0,
        }
    }

    /// Creates a QuatText instance with preallocated capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        let byte_capacity = (capacity + 3) / 4;
        Self {
            data: Vec::with_capacity(byte_capacity),
            len: 0,
        }
    }

    /// Appends a new Quat to the collection.
    pub fn push(&mut self, quat: Quat) {
        let quat_index = self.len % 4;
        if quat_index == 0 {
            self.data.push(0);
        }
        let byte_index = self.len / 4;
        let shift = (3 - quat_index) * 2;
        let val = (quat as u8) & 0b11;

        self.data[byte_index] &= !(0b11 << shift);
        self.data[byte_index] |= val << shift;
        self.len += 1;
    }

    /// Sets a Quat at the specified index.
    pub fn set(&mut self, index: usize, quat: Quat) -> Result<(), QuatError> {
        if index >= self.len {
            return Err(QuatError::IndexOutOfBounds { index, len: self.len });
        }
        let byte_index = index / 4;
        let quat_index = index % 4;
        let shift = (3 - quat_index) * 2;
        let val = (quat as u8) & 0b11;

        self.data[byte_index] &= !(0b11 << shift);
        self.data[byte_index] |= val << shift;
        Ok(())
    }

    pub fn as_slice(&self) -> QuatSlice<'_> {
       QuatSlice::new(&self.data, self.len)
    }

    /// Retrieves a Quat at the specified index.
    pub fn get(&self, index: usize) -> Result<Quat, QuatError> {
        if index >= self.len {
            return Err(QuatError::IndexOutOfBounds { index, len: self.len });
        }
        let byte_index = index / 4;
        let quat_index = index % 4;
        let shift = (3 - quat_index) * 2;
        let val = (self.data[byte_index] >> shift) & 0b11;

        Quat::try_from(val)
    }

    /// Returns the total number of stored quats.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Checks if the collection is empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Clears the contents of the collection.
    pub fn clear(&mut self) {
        self.data.clear();
        self.len = 0;
    }

    /// Shrinks the underlying buffer capacity to fit the current length.
    pub fn shrink_to_fit(&mut self) {
        self.data.shrink_to_fit();
    }

    /// Returns the raw underlying byte slice.
    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }
}

impl Default for QuatText {
    fn default() -> Self {
        Self::new()
    }
}

impl FromStr for QuatText {
    type Err = QuatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut quat_text = QuatText::with_capacity(s.len());
        for c in s.chars() {
            let quat = Quat::try_from(c)?;
            quat_text.push(quat);
        }
        Ok(quat_text)
    }
}

impl fmt::Display for QuatText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.len {
            if let Ok(quat) = self.get(i) {
                write!(f, "{}", quat)?;
            }
        }
        Ok(())
    }
}

// Collector implementation
impl FromIterator<Quat> for QuatText {
    fn from_iter<T: IntoIterator<Item = Quat>>(iter: T) -> Self {
        let mut quat_text = QuatText::new();
        for quat in iter {
            quat_text.push(quat);
        }
        quat_text
    }
}

impl Extend<Quat> for QuatText {
    fn extend<T: IntoIterator<Item = Quat>>(&mut self, iter: T) {
        for quat in iter {
            self.push(quat);
        }
    }
}

// Borrowed Iterator
pub struct QuatIter<'a> {
    quat_text: &'a QuatText,
    index: usize,
}

impl<'a> Iterator for QuatIter<'a> {
    type Item = Quat;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.quat_text.len() {
            let quat = self.quat_text.get(self.index).ok();
            self.index += 1;
            quat
        } else {
            None
        }
    }
}

impl<'a> IntoIterator for &'a QuatText {
    type Item = Quat;
    type IntoIter = QuatIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        QuatIter {
            quat_text: self,
            index: 0,
        }
    }
}

// Owned Iterator
pub struct IntoQuatIter {
    quat_text: QuatText,
    index: usize,
}

impl Iterator for IntoQuatIter {
    type Item = Quat;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.quat_text.len() {
            let quat = self.quat_text.get(self.index).ok();
            self.index += 1;
            quat
        } else {
            None
        }
    }
}

impl IntoIterator for QuatText {
    type Item = Quat;
    type IntoIter = IntoQuatIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoQuatIter {
            quat_text: self,
            index: 0,
        }
    }
}

impl<'a> QuatSlice<'a> {
    pub fn new(data: &'a [u8], len: usize) -> Self {
        Self { data, len }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn get(&self, index: usize) -> Result<Quat, QuatError> {
        if index >= self.len {
            return Err(QuatError::IndexOutOfBounds { index, len: self.len });
        }
        let byte_index = index / 4;
        let quat_index = index % 4;
        let shift = (3 - quat_index) * 2;
        let val = (self.data[byte_index] >> shift) & 0b11;
        Quat::try_from(val)
    }
}