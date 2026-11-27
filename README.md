# quat_text

`quat_text` is a lightweight, zero-dependency Rust library designed for memory-efficient text and data storage using **quaternary logic** (quats). By packing 4 quaternary values (each taking 2 bits: `00`, `01`, `10`, `11`) into a single standard byte (`u8`), it achieves an astounding **75% memory savings** compared to standard byte or character storage.

---

## Features

- **75% Memory Reduction**: Packs four 2-bit quats into every single byte.
- **Zero Dependencies**: Pure Rust standard library (`std`), ensuring blazing-fast compilation and maximum portability.
- **Ergonomic Macros**: Easily create collections using the custom `quat_text!` macro.
- **Robust Error Handling**: Clear custom errors (`QuatError`) for out-of-bound accesses, invalid bit patterns, and incorrect character parsing.
- **Idiomatic Rust Traits**: Full support for `FromStr`, `TryFrom`, `Display`, `Iterator`, `IntoIterator`, `FromIterator`, and `Extend`.
- **Modular Architecture**: Cleanly separated modules for quats, errors, text collection, and macros.
- **Quaternary Logic Operations**: Native bitwise operations (`&`, `\|`, `^`, `!`) for quaternary states.
- **Zero-Copy Slices (`QuatSlice`)**: Lightweight, heapless borrowed views for high-performance data processing.
- **Bioinformatics Integration**: Direct mapping for DNA/RNA nucleotides (`A`, `C`, `G`, `T/U`) into quaternary values.
- **SIMD-Optimized Utilities**: Fast vectorized pattern counting and text processing.
- **Built-in Benchmarks**: Integrated execution time tracking across all test suites.
- **Reverse Complement & Sequence Search**: Compute Watson-Crick complements, reverse complements (`to_reverse_complement`), RNA mappings (`to_rna`), and subsequence searches (`find_pattern`, `count_subsequence`) over packed sequences.
- **Raw Byte Interop & Slice Conversions**: Convert seamlessly between owned `QuatText`, raw byte buffers (`into_bytes`, `from_raw_parts`), and `QuatSlice` views (`as_quat_slice`, `to_quat_text`, `iter`).
- **Assignment Operators & String Equality**: Ergonomic in-place bitwise assignment (`&=`, `|=`, `^=`) on `Quat` as well as `PartialEq<&str>` comparisons on `QuatText`.

---

## Licensing

This project is dual-licensed:

* **Open Source (AGPLv3):** Free to use for open-source projects. If you use or modify this library in your software or network service, your source code must also be open-source under AGPLv3.
* **Commercial License:** Required if you want to use `quat_text` in closed-source or proprietary commercial software without AGPLv3 restrictions.

For commercial licensing, contact: **strahinjastojanovic826@gmail.com**

---

## Benchmark

# QuatText Library - Test & Benchmark Results (Debug Mode)

| # | Test Name | Status | Measured Time | Scope & What Is Verified |
| :-: | --- | :-: | :-: | --- |
| 01 | `test_01_empty_quat_text` | `ok` | `3.1µs` | Empty `QuatText::new()` initialization (`is_empty() == true`, `len() == 0`) |
| 02 | `test_02_push_single_quat` | `ok` | `5.3µs` | Pushing a single `Quat` (`len() == 1`, `is_empty() == false`, `get(0)`) |
| 03 | `test_03_byte_boundary_packing` | `ok` | `600ns` | 4 quats per byte packing and rollover onto a 2nd byte on the 5th quat |
| 04 | `test_04_from_str_parsing` | `ok` | `5.8µs` | Parsing `"0123"` via `FromStr` and verifying `V00..V11` values |
| 05 | `test_05_display_formatting` | `ok` | `4.9µs` | Formatting `QuatText` via `Display` (`to_string() == "3021"`) |
| 06 | `test_06_macro_usage` | `ok` | `1.5µs` | Ergonomic `quat_text!` macro usage with string and element list forms |
| 07 | `test_07_iterators_and_collection` | `ok` | `2.5µs` | `FromIterator` (`collect()`), `Extend` (`extend()`), and `IntoIterator` |
| 08 | `test_08_invalid_character_error` | `ok` | `700ns` | Error handling on invalid input (`"0143"` -> `QuatError::InvalidCharacter('4')`) |
| 09 | `test_09_index_out_of_bounds_error` | `ok` | `1.1µs` | Out-of-range index error (`QuatError::IndexOutOfBounds { index: 5, len: 2 }`) |
| 10 | `test_10_mutation_with_set` | `ok` | `2.7µs` | In-place mutation via `set(index, quat)` and `Display` verification |
| 11 | `test_11_large_scale_stress_and_memory_efficiency` | `ok` | `49.8µs` | 1,000 quats packed into 250 bytes (75% savings) + iterator integrity check |
| 12 | `test_12_complex_stressfull_bit_packing_and_mutations` | `ok` | `5.6µs` | Multi-byte mutations across bit shifts, `get()` checks, and `shrink_to_fit()` |
| 13 | `test_quaternary_logic_operations_with_benchmark` | `ok` | `200ns` | Bitwise `Quat` logic (`&`, `\|`, `^`, and `!`) |
| 14 | `test_quat_slice_zero_copy_with_benchmark` | `ok` | `700ns` | Zero-copy `QuatSlice` creation (`len()`, `is_empty()`, `get()`) |
| 15 | `test_bioinformatics_dna_mapping_with_benchmark` | `ok` | `400ns` | DNA nucleotide mapping (`Quat::from_dna` for `A/C/G/T` and `Quat::to_dna`) |
| 16 | `test_raw_parts_and_slice_conversions_with_benchmark` | `ok` | `2.6µs` | `into_bytes()`, `from_raw_parts()` (valid & invalid length), `as_quat_slice()`, `to_quat_text()`, `iter().len()` |
| 17 | `test_str_conversions_and_assign_ops_with_benchmark` | `ok` | `2.2µs` | `TryFrom<&str>`, `PartialEq<&str>`, and `&=` / `\|=` / `^=` assignment ops |
| 18 | `test_bio_ext_and_search_with_benchmark` | `ok` | `6µs` | `to_reverse_complement()`, `to_rna()`, `complement()`, `complement_in_place()`, `find_pattern()`, `count_subsequence()` |
| 19 | `test_swar_boundaries_with_benchmark` | `ok` | `10.7µs` | 64-bit SWAR counting across lengths `0..=137` (`0, 1, 3, 4, 7, 8, 15, 31, 32, 33, 64, 65, 137`) |
| 20 | `test_simd_pattern_counting_with_benchmark` | `ok` | `192µs` | SWAR pattern counting over 10,000 quats (`count_pattern_simd(Quat::V01) == 2500`) |

---

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
quat_text = "0.5.0"
```

---

## Testing

To start testing 

```toml
cargo test
```

---

## Quick Start

Here is a quick example demonstrating how to create, manipulate, and iterate over a `QuatText` collection:

```rust
use quat_text::{quat_text, Quat, QuatText};
use std::str::FromStr;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Create a collection using the ergonomic macro from string literal
    let mut qt = quat_text!("0123");
    
    assert_eq!(qt.len(), 4);
    assert_eq!(qt.get(0)?, Quat::V00);
    assert_eq!(qt.get(3)?, Quat::V11);

    // 2. Push new quaternary values dynamically
    qt.push(Quat::V10);
    assert_eq!(qt.len(), 5);

    // 3. Mutate values at specific indexes
    qt.set(1, Quat::V11)?;
    assert_eq!(qt.to_string(), "03232");

    // 4. Iterate over elements naturally
    for quat in &qt {
        println!("Quat value: {}", quat);
    }

    // 5. Verify memory efficiency (5 quats fit into just 2 bytes!)
    assert_eq!(qt.as_bytes().len(), 2);

    Ok(())
}
```

---

## Advanced Usage

### Parsing and Error Handling
You can safely parse standard strings into `QuatText`. Any invalid character (anything other than `'0'`, `'1'`, `'2'`, or `'3'`) will return a descriptive `QuatError`.

```rust
use quat_text::QuatText;
use std::str::FromStr;

match QuatText::from_str("0192") {
    Ok(qt) => println!("Parsed successfully: {}", qt),
    Err(e) => eprintln!("Parsing failed: {}", e), // Outputs invalid character error for '9'
}
```

### Collecting from Standard Iterators
`QuatText` implements `FromIterator` and `Extend`, allowing you to collect elements seamlessly from standard Rust iterators:

```rust
use quat_text::{Quat, QuatText};

let quats = vec![Quat::V01, Quat::V10, Quat::V11];
let qt: QuatText = quats.into_iter().filter(|&q| q != Quat::V01).collect();

assert_eq!(qt.len(), 2);
```

### Advanced Features & Bioinformatics

You can perform direct quaternary logic operations, use zero-copy slices, or map DNA/RNA sequences seamlessly:

```rust
use quat_text::{Quat, QuatText, QuatSlice};

// 1. Quaternary logic operations
let q1 = Quat::V01;
let q2 = Quat::V10;
let result_and = q1 & q2; // V00

// 2. Zero-copy slices for high performance
let qt = quat_text!("0123");
let slice = QuatSlice::new(qt.as_bytes(), qt.len());
assert_eq!(slice.get(3).unwrap(), Quat::V11);

// 3. Bioinformatics: DNA/RNA nucleotide mapping
let dna_base = Quat::from_dna('G').unwrap(); // Quat::V10
assert_eq!(dna_base.to_dna(), 'G');

### SIMD Counting, Raw Buffers, and Reverse Complement

```rust
use quat_text::bio::QuatBioExt;
use quat_text::{quat_text, Quat, QuatText};

let mut seq = quat_text!("01230123");
assert_eq!(seq.count_pattern_simd(Quat::V01), 2);

let pat = quat_text!("23");
assert_eq!(seq.find_pattern(&pat), Some(2));
assert_eq!(seq.count_subsequence(&pat), 2);

let rev_comp = seq.to_reverse_complement();
assert_eq!(rev_comp.len(), 8);

let len = seq.len();
let raw = seq.into_bytes();
let restored = QuatText::from_raw_parts(raw, len)?;
assert_eq!(restored.len(), 8);
# Ok::<(), quat_text::error::QuatError>(())