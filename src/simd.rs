use crate::text::QuatText;
use crate::quat::Quat;

impl QuatText {
    /// Counts occurrences of `target` using 64-bit SWAR (32 quats per step)
    /// over packed bytes, followed by a scalar tail loop.
    #[must_use]
    pub fn count_pattern_simd(&self, target: Quat) -> usize {
        let q_bits = (target as u8) & 0b11;
        let byte_splat = q_bits * 0b0101_0101;
        let word_splat = u64::from_ne_bytes([byte_splat; 8]);

        let full_bytes = self.len() / 4;
        let mut count = 0usize;
        let mut byte_idx = 0usize;

        // Process 8 packed bytes (32 quats) at a time
        while byte_idx + 8 <= full_bytes {
            let chunk: [u8; 8] = self.data[byte_idx..byte_idx + 8]
                .try_into()
                .expect("slice length is 8");
            let word = u64::from_ne_bytes(chunk);
            let diff = word ^ word_splat;

            // For each 2-bit lane `x`, `(x | (x >> 1)) & 0x5555...` has bit 0 set iff `x != 0`.
            // Inverting that mask over 2-bit lanes isolates matching quats.
            let non_zero_lanes = (diff | (diff >> 1)) & 0x5555_5555_5555_5555u64;
            let matching_lanes = (!non_zero_lanes) & 0x5555_5555_5555_5555u64;
            count += matching_lanes.count_ones() as usize;

            byte_idx += 8;
        }

        // Remaining full bytes (< 8)
        while byte_idx < full_bytes {
            let b = self.data[byte_idx];
            let diff = b ^ byte_splat;
            let non_zero = (diff | (diff >> 1)) & 0b0101_0101;
            let matching = (!non_zero) & 0b0101_0101;
            count += matching.count_ones() as usize;
            byte_idx += 1;
        }

        // Trailing partial byte (< 4 quats at the end of `self.len()`)
        let rem_quats = self.len() % 4;
        if rem_quats > 0 {
            let b = self.data[full_bytes];
            for q_idx in 0..rem_quats {
                let shift = (3 - q_idx) * 2;
                if ((b >> shift) & 0b11) == q_bits {
                    count += 1;
                }
            }
        }

        count
    }

}