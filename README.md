
# bbse — Backward Binary Search Encoding

`bbse` is a minimal and deterministic encoding scheme for values in a sorted integer range.  
It encodes a target value as a binary decision path, following the steps of binary search.  

This approach provides a **prefix-free**, **compact**, and **reversible** representation of values  
from any `[start, end)` interval.

---

## ✨ Features

- Prefix-free binary encoding for any sorted range
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

## 🎨 Use case: color encoding

The algorithm was originally inspired by the need to encode color deltas efficiently
in an experimental lossless image format. Each delta channel (R/G/B) was encoded
as a binary search path, taking advantage of sorted delta distributions.

This enabled compact per-channel encoding with **minimal logic**, without relying on entropy coding.

---

## 📦 Installation

```toml
[dependencies]
bbse = "0.1.0"
```

(Add this to your `Cargo.toml` after publication)

---

## 📄 License

MIT

---

## 🔬 Future extensions

* Used in image codec prototype to encode color deltas via BBSE for predictable bit-length.

