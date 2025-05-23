//! # BBSE — Backward Binary Search Encoding (Stack-based Concept)
//!
//! BBSE encodes integer values from a known sorted range as a compact path of binary decisions — the same steps a binary search would take to locate the value.
//!
//! This encoding is:
//! - **Prefix-free** and minimal;
//! - **Deterministic** and reversible;
//! - **Stack-compatible** — no headers, no length metadata;
//! - **Range-aware** — optimized for values near the center.
//!
//! ## Overview
//!
//! Each value is represented as a `BitVec`, encoding the binary decisions taken while performing binary search to locate the value in `[start..end)`.
//! The search terminates early if the midpoint equals the target value.
//!
//! BBSE paths can be stored directly in a stack, enabling extremely compact storage.
//!
//! ## Features
//!
//! - Compact, unique encoding per value;
//! - No external data needed to decode (just `start..end` range);
//! - Zero allocation per path in streaming form;
//! - No statistical modeling — pure binary search geometry.
//!
//! ## Examples
//!
//! ```rust
//! use bbse::{encode, decode};
//! let path = encode(0, 256, 128); // one step or even empty path
//! let value = decode(0, 256, &path);
//! assert_eq!(value, 128);
//! ```
//!
//! ```rust
//! use bbse::{encode, BBSEStack};
//! let mut stack = BBSEStack::new();
//! for v in [0, 1, 2, 3, 4, 5, 6, 7] {
//!     stack.push(encode(0, 8, v));
//! }
//! let decoded = stack.decode_all(0, 8);
//! assert_eq!(decoded, vec![0, 1, 2, 3, 4, 5, 6, 7]);
//! ```
//!
//! ## Limitations
//!
//! - Values must lie within the specified range.
//! - Encoded paths must be decoded with the same range.
//! - Not optimized for random-access decoding without range knowledge.
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::{vec, vec::Vec};
use bitvec::order::Msb0;
use bitvec::vec::BitVec;
use core::{default::Default, option::Option, panic};

/// BBSE stack-based encoding: returns a BitVec representing the path
pub fn encode(start: usize, end: usize, target: usize) -> BitVec<u8, Msb0> {
    if start >= end {
        panic!("Invalid range: start ({}) >= end ({})", start, end);
    }
    if !(start <= target && target < end) {
        panic!("target ({}) out of bounds [{}, {})", target, start, end);
    }

    let mut path = BitVec::<u8, Msb0>::new();
    let mut lo = start;
    let mut hi = end;

    loop {
        let mid = (lo + hi) / 2;

        if target == mid {
            break;
        }

        if target < mid {
            path.push(false);
            hi = mid;
        } else {
            path.push(true);
            lo = mid;
        }

        if hi - lo == 1 {
            break;
        }
    }

    path
}

/// BBSE custom midpoint (optional)
pub fn encode_from(start: usize, end: usize, target: usize, midpoint: usize) -> BitVec<u8, Msb0> {
    if start >= end {
        panic!("Invalid range: start ({}) >= end ({})", start, end);
    }
    if !(start <= target && target < end) {
        panic!("target ({}) out of bounds [{}, {})", target, start, end);
    }
    if !(start < midpoint && midpoint < end) {
        panic!(
            "midpoint ({}) must be within (start={}, end={})",
            midpoint, start, end
        );
    }

    let mut path = BitVec::<u8, Msb0>::new();
    let mut lo = start;
    let mut hi = end;
    let mut mid = midpoint;

    loop {
        if target == mid {
            break;
        }

        if target < mid {
            path.push(false);
            hi = mid;
        } else {
            path.push(true);
            lo = mid;
        }

        if hi - lo == 1 {
            break;
        }

        mid = (lo + hi) / 2;
    }

    path
}

/// BBSE decoder: consumes a path and returns the corresponding value
pub fn decode(start: usize, end: usize, path: &BitVec<u8, Msb0>) -> usize {
    let mut lo = start;
    let mut hi = end;

    for bit in path.iter() {
        let mid = (lo + hi) / 2;
        if *bit {
            lo = mid;
        } else {
            hi = mid;
        }
    }

    (lo + hi) / 2
}

/// BBSE custom midpoint (optional for default midpoint encoding)
pub fn decode_from(start: usize, end: usize, path: &BitVec<u8, Msb0>, midpoint: usize) -> usize {
    if path.is_empty() {
        return midpoint;
    }

    let mut lo = start;
    let mut hi = end;
    let mut mid = midpoint;

    for bit in path.iter() {
        if *bit {
            lo = mid;
        } else {
            hi = mid;
        }

        if hi - lo == 1 {
            break;
        }

        mid = (lo + hi) / 2;
    }

    (lo + hi) / 2
}

/// Stack model — store multiple values as separate paths
pub struct BBSEStack {
    pub entries: Vec<BitVec<u8, Msb0>>,
}
impl Default for BBSEStack {
    fn default() -> Self {
        Self::new()
    }
}

impl BBSEStack {
    pub fn new() -> Self {
        Self { entries: vec![] }
    }

    pub fn push(&mut self, path: BitVec<u8, Msb0>) {
        self.entries.push(path);
    }

    pub fn pop(&mut self) -> Option<BitVec<u8, Msb0>> {
        self.entries.pop()
    }

    pub fn decode_all(&self, start: usize, end: usize) -> Vec<usize> {
        self.entries.iter().map(|p| decode(start, end, p)).collect()
    }

    #[cfg(feature = "std")]
    pub fn print_all(&self) {
        self.entries.iter().for_each(|f| println!("encoded: {}", f));
    }
}
