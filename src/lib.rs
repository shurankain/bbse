//! # bbse — Backward Binary Search Encoding
//!
//! A minimal and deterministic encoding scheme for any sorted range,
//! based on walking the binary search path to a target value.
//!
//! This encoding is useful when you need:
//!
//! - a compact, prefix-free representation of integers within a known range;
//! - reversible and stateless encoding for sorted domains;
//! - deterministic paths suitable for compression, indexing or entropy coding;
//! - simple decoding without precomputed tables (unlike Elias or Fano).
//!
//! ## Features
//!
//! - Supports custom midpoint selection (`encode_from`) for skewed distributions;
//! - Safe and panic-resistant with clear invariants;
//! - Compatible with [`no_std`] with `alloc` (planned).
//!
//! ## Examples
//!
//! Basic usage:
//!
//! ```
//! use bbse::{encode, decode};
//! let path = encode(0, 16, 5);
//! let value = decode(0, 16, &path);
//! assert_eq!(value, 5);
//! ```
//!
//! With a custom midpoint:
//!
//! ```
//! use bbse::{encode_from, decode};
//! let path = encode_from(0, 16, 5, 8); // start at 8 instead of default midpoint
//! let value = decode(0, 16, &path);
//! assert_eq!(value, 5);
//! ```
//!
//! ## Behavior
//!
//! - `encode` produces a sequence of binary decisions (`BitVec`) that
//!   represents the search path for `target` in `[start..end)`.
//! - `decode` reverses that path to recover the original value.
//! - The encoded path is **prefix-free** and **uniquely decodable**.
//!
//! ## Limitations
//!
//! - Panics if `target` is out of range;
//! - Panics if the path is invalid or doesn't narrow the range to a single value;
//! - Only works for discrete ranges over `usize`.
#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
extern crate core;

use bitvec::order::Msb0;
use bitvec::vec::BitVec;
use core::panic;

/// Encodes a value using a custom initial midpoint instead of the default center.
/// Useful for biased distributions toward one end of the range.
///
/// # Panics
/// Panics if `midpoint` is not within the range or causes unbalanced division.
pub fn encode_from(start: usize, end: usize, target: usize, midpoint: usize) -> BitVec<u8, Msb0> {
    if start >= end {
        panic!("Invalid range");
    }
    if !(start <= target && target < end) {
        panic!("target out of bounds");
    }

    let mut path = BitVec::<u8, Msb0>::new();
    let mut left = start;
    let mut right = end;

    if right - left <= 1 {
        return path;
    }

    let mut mid = midpoint;
    if !(left < mid && mid < right) {
        panic!("midpoint must be within (start, end)");
    }

    while right - left > 1 {
        if target < mid {
            path.push(false);
            right = mid;
        } else {
            path.push(true);
            left = mid;
        }

        let new_range = right - left;
        if new_range == 1 {
            break;
        }

        mid = (left + right) / 2;
    }

    path
}

/// Convenience method that assumes midpoint is the center of the range.
pub fn encode(start: usize, end: usize, target: usize) -> BitVec<u8, Msb0> {
    let midpoint = (start + end) / 2;
    encode_from(start, end, target, midpoint)
}

/// Decodes a binary decision sequence into the value it encodes,
/// given the original [`start`, `end`) range.
///
/// # Panics
/// Panics if the path is invalid or incomplete.
pub fn decode(mut start: usize, mut end: usize, path: &BitVec<u8, Msb0>) -> usize {
    for bit in path {
        let mid = (start + end) / 2;
        if *bit {
            start = mid;
        } else {
            end = mid;
        }
    }
    if start + 1 != end {
        panic!("incomplete or invalid path");
    }
    start
}
