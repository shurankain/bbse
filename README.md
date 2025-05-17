# bbse — Backward Binary Search Encoding

[![Crates.io](https://img.shields.io/crates/v/bbse.svg)](https://crates.io/crates/bbse)

`bbse` is a minimal and deterministic encoding scheme for values in a sorted integer range.  
It encodes a target value as a binary decision path, following the steps of binary search.  

This approach provides a **prefix-free**, **compact**, and **reversible** representation of values  
from any `[start, end)` interval.

---

## ✨ Features

- Prefix-free binary encoding for any sorted range
- Customizable midpoint for biased distributions
- Simple, deterministic, and lossless
- Suitable for compression, range indexing, embedded systems
- No heap allocation (except for returned bitvector)

---

## 🚀 Example

```rust
use bbse::{encode, decode};

let bits = encode(0, 16, 5);        // e.g. [true, false, true]
let value = decode(0, 16, &bits);   // -> 5
assert_eq!(value, 5);
````

---

## 🎯 Custom midpoint

For skewed distributions (e.g., values near the beginning or end),
you can control the **first midpoint** to reduce average path length:

```rust
use bbse::{encode_from, decode};

let bits = encode_from(0, 16, 3, 4);  // start=0, end=16, target=3, midpoint=4
let value = decode(0, 16, &bits);
assert_eq!(value, 3);
```

This gives you **greater control over compression performance**.

---

## 🎨 Use case: color encoding

The algorithm was originally inspired by the need to encode color deltas efficiently
in an experimental lossless image format. Each delta channel (R/G/B) was encoded
as a binary search path, taking advantage of sorted delta distributions.

This enabled compact per-channel encoding with **minimal logic**, without relying on entropy coding.

---

## 📦 Installation

```toml
[dependencies]
bbse = "0.2.0"
```

---

## 📄 License

MIT

---

## 🔮 Future directions

* Optional `no_std` support for embedded targets
