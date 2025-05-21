# bbse — Backward Binary Search Encoding (v2.0)

[![Crates.io](https://img.shields.io/crates/v/bbse.svg)](https://crates.io/crates/bbse)

`bbse` encodes integer values as the path that binary search would take to find them in a known range.  
The result is a **prefix-free**, **compact**, **reversible**, and **range-aware** representation —  
ideal for low-footprint use cases like compression, embedded indexing, and color deltas.

---

## ✨ Highlights

- 🧠 Path-based encoding using binary search logic
- ✅ Prefix-free, minimal-length representation
- 🪆 Stack-compatible — values can be stored without headers or offsets
- 🧮 Customizable midpoint for biased distributions
- 🚫 No statistical model or table required
- 🧵 `no_std` compatible with `alloc`

---

## 🚀 Quick Example

```rust
use bbse::{encode, decode};

let bits = encode(0, 256, 128); // Path to 128 in [0, 256)
let value = decode(0, 256, &bits);
assert_eq!(value, 128);
````

---

## 🎯 Stack-based Encoding

Each encoded value is just a binary search path — ideal for use as a stack of values:

```rust
use bbse::{encode, BBSEStack};

let mut stack = BBSEStack::new();
for value in [0, 1, 2, 3, 4, 5, 6, 7] {
    stack.push(encode(0, 8, value));
}

let decoded = stack.decode_all(0, 8);
assert_eq!(decoded, vec![0, 1, 2, 3, 4, 5, 6, 7]);
```

---

## 🛠 Custom Midpoint (Optional)

```rust
use bbse::{encode_from, decode};

let bits = encode_from(0, 16, 3, 4);  // Use midpoint = 4 instead of center
let value = decode(0, 16, &bits);
assert_eq!(value, 3);
```

---

## 🎨 Origin: Efficient Color Deltas

This project originated while designing a custom image codec for RGB delta compression.
By encoding deltas using binary search paths instead of entropy coding, we achieved:

* Predictable bit lengths
* Simple bitstream merging
* Ultra-lightweight decoding with no tables or models

---

## 📦 Installation

>MSRV (Minimum Supported Rust Version) This crate requires **Rust 1.78.0** or later.

```toml
[dependencies]
bbse = "2.0.0"
```

For embedded or `no_std` use:

```toml
[dependencies.bbse]
version = "2.0.0"
default-features = false
```

---

## ⚙️ Features

* `std` (default): Enables printing and full integration with standard I/O
* `no_std`: Disables `std`, uses `alloc` only — ideal for embedded targets

---

**BBSE is simple, elegant, and inspired by the structure of the data itself — not statistics.**
No entropy. No overhead. Just binary logic.

---

## 📄 License

MIT
