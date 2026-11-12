pub mod error;
pub mod macros;
pub mod quat;
pub mod text;
pub mod bio;
pub mod simd;

pub use error::QuatError;
pub use quat::Quat;
pub use text::{IntoQuatIter, QuatIter, QuatSlice, QuatText};

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    use std::time::Instant;

    // ==========================================
    // 1. NAJLAKŠI TESTOVI (Osnovna stanja)
    // ==========================================

    #[test]
    fn test_01_empty_quat_text() {
        let start = Instant::now();

        let qt = QuatText::new();
        assert!(qt.is_empty());
        assert_eq!(qt.len(), 0);

        let duration = start.elapsed();
        println!("[BENCHMARK] test_01_empty_quat_text took: {:?}", duration);
    }

    #[test]
    fn test_02_push_single_quat() {
        let start = Instant::now();

        let mut qt = QuatText::new();
        qt.push(Quat::V01);
        assert_eq!(qt.len(), 1);
        assert!(!qt.is_empty());
        assert_eq!(qt.get(0).unwrap(), Quat::V01);

        let duration = start.elapsed();
        println!("[BENCHMARK] test_02_push_single_quat took: {:?}", duration);
    }

    // ==========================================
    // 2. INTERMEDIATE TESTS (Boundaries and Parsing)
    // ==========================================

    #[test]
    fn test_03_byte_boundary_packing() {
        let start = Instant::now();

        let mut qt = QuatText::new();
        // 4 quats fit into exactly 1 byte
        qt.push(Quat::V00);
        qt.push(Quat::V01);
        qt.push(Quat::V10);
        qt.push(Quat::V11);
        // 5th quat opens a new byte
        qt.push(Quat::V01);

        assert_eq!(qt.len(), 5);
        assert_eq!(qt.as_bytes().len(), 2); // 2 bytes allocated
        assert_eq!(qt.get(3).unwrap(), Quat::V11);
        assert_eq!(qt.get(4).unwrap(), Quat::V01);

        let duration = start.elapsed();
        println!("[BENCHMARK] test_03_byte_boundary_packing took: {:?}", duration);
    }

    #[test]
    fn test_04_from_str_parsing() {
        let start = Instant::now();

        let qt = QuatText::from_str("0123").unwrap();
        assert_eq!(qt.len(), 4);
        assert_eq!(qt.get(0).unwrap(), Quat::V00);
        assert_eq!(qt.get(1).unwrap(), Quat::V01);
        assert_eq!(qt.get(2).unwrap(), Quat::V10);
        assert_eq!(qt.get(3).unwrap(), Quat::V11);

        let duration = start.elapsed();
        println!("[BENCHMARK] test_04_from_str_parsing took: {:?}", duration);
    }

    #[test]
    fn test_05_display_formatting() {
        let start = Instant::now();

        let mut qt = QuatText::new();
        qt.push(Quat::V11);
        qt.push(Quat::V00);
        qt.push(Quat::V10);
        qt.push(Quat::V01);
        assert_eq!(qt.to_string(), "3021");

        let duration = start.elapsed();
        println!("[BENCHMARK] test_05_display_formatting took: {:?}", duration);
    }

    #[test]
    fn test_06_macro_usage() {
        let start = Instant::now();

        let qt1 = QuatText::from_str("0123").unwrap();
        assert_eq!(qt1.len(), 4);
        assert_eq!(qt1.get(2).unwrap(), Quat::V10);

        let qt2 = quat_text![Quat::V00, Quat::V11, Quat::V10];
        assert_eq!(qt2.len(), 3);
        assert_eq!(qt2.get(1).unwrap(), Quat::V11);

        let duration = start.elapsed();
        println!("[BENCHMARK] test_06_macro_usage took: {:?}", duration);
    }

    // ==========================================
    // 3. ADVANCED TESTS (Iterators and Errors)
    // ==========================================

    #[test]
    fn test_07_iterators_and_collection() {
        let start = Instant::now();

        let initial: Vec<Quat> = vec![Quat::V00, Quat::V01];
        let mut qt: QuatText = initial.into_iter().collect();
        assert_eq!(qt.len(), 2);

        qt.extend(vec![Quat::V10, Quat::V11]);
        assert_eq!(qt.len(), 4);

        let collected: Vec<Quat> = qt.into_iter().collect();
        assert_eq!(collected, vec![Quat::V00, Quat::V01, Quat::V10, Quat::V11]);

        let duration = start.elapsed();
        println!("[BENCHMARK] test_07_iterators_and_collection took: {:?}", duration);
    }

    #[test]
    fn test_08_invalid_character_error() {
        let start = Instant::now();

        let res: Result<QuatText, QuatError> = QuatText::from_str("0143"); // '4' is not a valid quat
        assert!(matches!(res, Err(QuatError::InvalidCharacter('4'))));

        let duration = start.elapsed();
        println!("[BENCHMARK] test_08_invalid_character_error took: {:?}", duration);
    }

    #[test]
    fn test_09_index_out_of_bounds_error() {
        let start = Instant::now();

        let qt = QuatText::from_str("01").unwrap();
        let err = qt.get(5).unwrap_err();
        assert!(matches!(err, QuatError::IndexOutOfBounds { index: 5, len: 2 }));

        let duration = start.elapsed();
        println!("[BENCHMARK] test_09_index_out_of_bounds_error took: {:?}", duration);
    }

    #[test]
    fn test_10_mutation_with_set() {
        let start = Instant::now();

        let mut qt = QuatText::from_str("0000").unwrap();
        qt.set(2, Quat::V11).unwrap();
        assert_eq!(qt.get(2).unwrap(), Quat::V11);
        assert_eq!(qt.to_string(), "0030");

        let duration = start.elapsed();
        println!("[BENCHMARK] test_10_mutation_with_set took: {:?}", duration);
    }

    // ==========================================
    // 4. STRESS TESTS (Massive data and bit-shifting)
    // ==========================================

    #[test]
    fn test_11_large_scale_stress_and_memory_efficiency() {
        let start = Instant::now();

        let mut qt = QuatText::with_capacity(1000);
        for i in 0..1000 {
            let q = match i % 4 {
                0 => Quat::V00,
                1 => Quat::V01,
                2 => Quat::V10,
                _ => Quat::V11,
            };
            qt.push(q);
        }
        assert_eq!(qt.len(), 1000);
        // Verify 75% memory savings: 1000 quats * 2 bits = 2000 bits = exactly 250 bytes
        assert_eq!(qt.as_bytes().len(), 250);

        // Validate integrity of all 1000 elements through the iterator
        for (i, quat) in qt.into_iter().enumerate() {
            let expected = match i % 4 {
                0 => Quat::V00,
                1 => Quat::V01,
                2 => Quat::V10,
                _ => Quat::V11,
            };
            assert_eq!(quat, expected);
        }

        let duration = start.elapsed();
        println!("[BENCHMARK] test_11_large_scale_stress_and_memory_efficiency took: {:?}", duration);
    }

    #[test]
    fn test_12_complex_stressfull_bit_packing_and_mutations() {
        let start = Instant::now();

        let mut qt = QuatText::new();
        // Initialize with 20 elements (5 bytes)
        for _ in 0..20 {
            qt.push(Quat::V00);
        }

        // Aggressively change values at different positions intersecting various bytes and shifts
        qt.set(0, Quat::V11).unwrap();
        qt.set(3, Quat::V10).unwrap();
        qt.set(4, Quat::V01).unwrap();
        qt.set(7, Quat::V11).unwrap();
        qt.set(12, Quat::V10).unwrap();
        qt.set(19, Quat::V11).unwrap();

        // Verify accuracy of updates after complex bit-packing
        assert_eq!(qt.get(0).unwrap(), Quat::V11);
        assert_eq!(qt.get(3).unwrap(), Quat::V10);
        assert_eq!(qt.get(4).unwrap(), Quat::V01);
        assert_eq!(qt.get(7).unwrap(), Quat::V11);
        assert_eq!(qt.get(12).unwrap(), Quat::V10);
        assert_eq!(qt.get(19).unwrap(), Quat::V11);

        // Stress test reallocation and shrinking memory
        qt.shrink_to_fit();
        assert_eq!(qt.len(), 20);

        let duration = start.elapsed();
        println!("[BENCHMARK] test_12_complex_stressfull_bit_packing_and_mutations took: {:?}", duration);
    }

    #[test]
    fn test_quaternary_logic_operations_with_benchmark() {
        let start = Instant::now();

        let q1 = Quat::V01; // 01
        let q2 = Quat::V10; // 10

        // Test Bitwise AND
        let res_and = q1 & q2;
        assert_eq!(res_and, Quat::V00);

        // Test Bitwise OR
        let res_or = q1 | q2;
        assert_eq!(res_or, Quat::V11);

        // Test Bitwise XOR
        let res_xor = q1 ^ q2;
        assert_eq!(res_xor, Quat::V11);

        // Test Bitwise NOT (Quaternary complement)
        let res_not = !q1;
        assert_eq!(res_not, Quat::V10);

        let duration = start.elapsed();
        println!("[BENCHMARK] Quaternary logic operations took: {:?}", duration);
    }

    #[test]
    fn test_quat_slice_zero_copy_with_benchmark() {
        let start = Instant::now();

        let mut qt = QuatText::new();
        qt.push(Quat::V00);
        qt.push(Quat::V01);
        qt.push(Quat::V10);
        qt.push(Quat::V11);

        // Create a zero-copy slice from raw bytes
        let slice = QuatSlice::new(qt.as_bytes(), qt.len());

        assert_eq!(slice.len(), 4);
        assert!(!slice.is_empty());
        assert_eq!(slice.get(0).unwrap(), Quat::V00);
        assert_eq!(slice.get(3).unwrap(), Quat::V11);

        let duration = start.elapsed();
        println!("[BENCHMARK] QuatSlice zero-copy operations took: {:?}", duration);
    }

    #[test]
    fn test_bioinformatics_dna_mapping_with_benchmark() {
        let start = Instant::now();

        // Test DNA to Quat conversion (A=00, C=01, G=10, T=11)
        let a_quat = Quat::from_dna('A').unwrap();
        let c_quat = Quat::from_dna('C').unwrap();
        let g_quat = Quat::from_dna('G').unwrap();
        let t_quat = Quat::from_dna('T').unwrap();

        assert_eq!(a_quat, Quat::V00);
        assert_eq!(c_quat, Quat::V01);
        assert_eq!(g_quat, Quat::V10);
        assert_eq!(t_quat, Quat::V11);

        // Test Quat back to DNA character
        assert_eq!(Quat::V00.to_dna(), 'A');
        assert_eq!(Quat::V11.to_dna(), 'T');

        let duration = start.elapsed();
        println!("[BENCHMARK] Bioinformatics DNA mapping took: {:?}", duration);
    }

    #[test]
    fn test_simd_pattern_counting_with_benchmark() {
        let start = Instant::now();

        let mut qt = QuatText::with_capacity(10000);
        for i in 0..10000 {
            let q = match i % 4 {
                0 => Quat::V00,
                1 => Quat::V01,
                2 => Quat::V10,
                _ => Quat::V11,
            };
            qt.push(q);
        }

        // Count occurrences using vector/simd optimized path
        let count = qt.count_pattern_simd(Quat::V01);
        assert_eq!(count, 2500);

        let duration = start.elapsed();
        println!("[BENCHMARK] SIMD pattern counting (10,000 items) took: {:?}", duration);
    }

}