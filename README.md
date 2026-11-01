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

---

## Licensing

This project is dual-licensed:

* **Open Source (AGPLv3):** Free to use for open-source projects. If you use or modify this library in your software or network service, your source code must also be open-source under AGPLv3.
* **Commercial License:** Required if you want to use `quat_text` in closed-source or proprietary commercial software without AGPLv3 restrictions.

For commercial licensing, contact: **strahinjastojanovic826@gmail.com**

---

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
quat_text = "0.3.0"
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