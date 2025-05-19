# bbse — Backward Binary Search Encoding

[![Crates.io](https://img.shields.io/crates/v/bbse.svg)](https://crates.io/crates/bbse)

`bbse` is a minimal and deterministic encoding scheme for values in a sorted integer range.  
It encodes a target value as a binary decision path, following the steps of binary search.

This results in a **prefix-free**, **compact**, and **reversible** representation of any value  
within a known interval `[start, end)`.

---

## ✨ Features

- Prefix-free binary encoding for sorted domains
- Customizable midpoint to optimize for skewed distributions
- Constant-time decoding without lookup tables
- `no_std` compatible with `alloc`
- Suitable for compression, indexing, image deltas, and embedded systems

---

## 🚀 Basic example

```rust
use bbse::{encode, decode};

let bits = encode(0, 16, 5);        // Binary search path for 5 in [0, 16)
let value = decode(0, 16, &bits);   // -> 5
assert_eq!(value, 5);
````

---

## 🎯 Custom midpoint

You can manually specify the first midpoint to better handle non-uniform distributions:

```rust
use bbse::{encode_from, decode};

let bits = encode_from(0, 16, 3, 4);  // Midpoint = 4 instead of 8
let value = decode(0, 16, &bits);
assert_eq!(value, 3);
```

This gives more control over the generated bit length.

---

## 🎨 Real-world use case: color encoding

The core algorithm was inspired by the problem of efficiently encoding **color deltas**
in a lossless image codec. By encoding each delta value (R, G, B) as a binary search path
within a bounded range, we achieved lightweight compression with predictable bit lengths
and minimal branching.

This avoids full entropy coding while still producing short bitstreams.

---

## 📦 Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
bbse = "0.2.2"
```

---

## 🛠 no\_std support

This crate supports `#![no_std]` environments using the `alloc` crate:

```toml
[dependencies.bbse]
version = "0.2.2"
default-features = false
```

---

## 📄 License

MIT
