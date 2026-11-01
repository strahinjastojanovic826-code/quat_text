pub mod error;
pub mod macros;
pub mod quat;
pub mod text;

pub use error::QuatError;
pub use quat::Quat;
pub use text::{IntoQuatIter, QuatIter, QuatText};

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    // ==========================================
    // 1. NAJLAKŠI TESTOVI (Osnovna stanja)
    // ==========================================

    #[test]
    fn test_01_empty_quat_text() {
        let qt = QuatText::new();
        assert!(qt.is_empty());
        assert_eq!(qt.len(), 0);
    }

    #[test]
    fn test_02_push_single_quat() {
        let mut qt = QuatText::new();
        qt.push(Quat::V01);
        assert_eq!(qt.len(), 1);
        assert!(!qt.is_empty());
        assert_eq!(qt.get(0).unwrap(), Quat::V01);
    }

    // ==========================================
    // 2. SREDNJI NIVO TESTOVA (Granice i parsiranje)
    // ==========================================

    #[test]
    fn test_03_byte_boundary_packing() {
        let mut qt = QuatText::new();
        // 4 quat-a staju u tačno 1 bajt
        qt.push(Quat::V00);
        qt.push(Quat::V01);
        qt.push(Quat::V10);
        qt.push(Quat::V11);
        // 5. quat otvara novi bajt
        qt.push(Quat::V01);

        assert_eq!(qt.len(), 5);
        assert_eq!(qt.as_bytes().len(), 2); // 2 bajta alocirana
        assert_eq!(qt.get(3).unwrap(), Quat::V11);
        assert_eq!(qt.get(4).unwrap(), Quat::V01);
    }

    #[test]
    fn test_04_from_str_parsing() {
        let qt = QuatText::from_str("0123").unwrap();
        assert_eq!(qt.len(), 4);
        assert_eq!(qt.get(0).unwrap(), Quat::V00);
        assert_eq!(qt.get(1).unwrap(), Quat::V01);
        assert_eq!(qt.get(2).unwrap(), Quat::V10);
        assert_eq!(qt.get(3).unwrap(), Quat::V11);
    }

    #[test]
    fn test_05_display_formatting() {
        let mut qt = QuatText::new();
        qt.push(Quat::V11);
        qt.push(Quat::V00);
        qt.push(Quat::V10);
        qt.push(Quat::V01);
        assert_eq!(qt.to_string(), "3021");
    }

    #[test]
    fn test_06_macro_usage() {
        let qt1 = QuatText::from_str("0123").unwrap();
        assert_eq!(qt1.len(), 4);
        assert_eq!(qt1.get(2).unwrap(), Quat::V10);

        let qt2 = quat_text![Quat::V00, Quat::V11, Quat::V10];
        assert_eq!(qt2.len(), 3);
        assert_eq!(qt2.get(1).unwrap(), Quat::V11);
    }

    // ==========================================
    // 3. NAPREDNI TESTOVI (Iteratori i greške)
    // ==========================================

    #[test]
    fn test_07_iterators_and_collection() {
        let initial = vec![Quat::V00, Quat::V01];
        let mut qt: QuatText = initial.into_iter().collect();
        assert_eq!(qt.len(), 2);

        qt.extend(vec![Quat::V10, Quat::V11]);
        assert_eq!(qt.len(), 4);

        let collected: Vec<Quat> = qt.into_iter().collect();
        assert_eq!(collected, vec![Quat::V00, Quat::V01, Quat::V10, Quat::V11]);
    }

    #[test]
    fn test_08_invalid_character_error() {
        let res = QuatText::from_str("0143"); // '4' nije validan kvat
        assert!(matches!(res, Err(QuatError::InvalidCharacter('4'))));
    }

    #[test]
    fn test_09_index_out_of_bounds_error() {
        let qt = QuatText::from_str("01").unwrap();
        let err = qt.get(5).unwrap_err();
        assert!(matches!(err, QuatError::IndexOutOfBounds { index: 5, len: 2 }));
    }

    #[test]
    fn test_10_mutation_with_set() {
        let mut qt = QuatText::from_str("0000").unwrap();
        qt.set(2, Quat::V11).unwrap();
        assert_eq!(qt.get(2).unwrap(), Quat::V11);
        assert_eq!(qt.to_string(), "0030");
    }

    // ==========================================
    // 4. NAJSTRESIBNI / STRESS TESTOVI (Masivni podaci i bit-shifting)
    // ==========================================

    #[test]
    fn test_11_large_scale_stress_and_memory_efficiency() {
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
        // Provera 75% uštede memorije: 1000 kvat-a * 2 bita = 2000 bita = tačno 250 bajtova
        assert_eq!(qt.as_bytes().len(), 250);

        // Validacija integriteta svih 1000 elemenata kroz iterator
        for (i, quat) in qt.into_iter().enumerate() {
            let expected = match i % 4 {
                0 => Quat::V00,
                1 => Quat::V01,
                2 => Quat::V10,
                _ => Quat::V11,
            };
            assert_eq!(quat, expected);
        }
    }

    #[test]
    fn test_12_complex_stressful_bit_packing_and_mutations() {
        let mut qt = QuatText::new();
        // Inicijalizujemo sa 20 elemenata (5 bajtova)
        for _ in 0..20 {
            qt.push(Quat::V00);
        }

        // Agresivno menjamo vrednosti na različitim pozicijama koje presecaju različite bajtove i šiftranja
        qt.set(0, Quat::V11).unwrap();
        qt.set(3, Quat::V10).unwrap();
        qt.set(4, Quat::V01).unwrap();
        qt.set(7, Quat::V11).unwrap();
        qt.set(12, Quat::V10).unwrap();
        qt.set(19, Quat::V11).unwrap();

        // Proveravamo tačnost upisa nakon kompleksnog bitovnog pakovanja
        assert_eq!(qt.get(0).unwrap(), Quat::V11);
        assert_eq!(qt.get(3).unwrap(), Quat::V10);
        assert_eq!(qt.get(4).unwrap(), Quat::V01);
        assert_eq!(qt.get(7).unwrap(), Quat::V11);
        assert_eq!(qt.get(12).unwrap(), Quat::V10);
        assert_eq!(qt.get(19).unwrap(), Quat::V11);

        // Stres testiranje realociranja i skupljanja memorije
        qt.shrink_to_fit();
        assert_eq!(qt.len(), 20);
    }
}