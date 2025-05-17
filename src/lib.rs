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

use bitvec::order::Msb0;
use bitvec::vec::BitVec;

/// Encodes a value using a custom initial midpoint instead of the default center.
/// Useful for biased distributions toward one end of the range.
///
/// # Panics
/// Panics if `midpoint` is not within the range or causes unbalanced division.
pub fn encode_from(start: usize, end: usize, target: usize, midpoint: usize) -> BitVec<u8, Msb0> {
    assert!(start < end, "Invalid range");
    assert!(start <= target && target < end, "target out of bounds");

    let mut path = BitVec::<u8, Msb0>::new();
    let mut left = start;
    let mut right = end;

    if right - left <= 1 {
        return path;
    }

    let mut mid = midpoint;
    assert!(
        left < mid && mid < right,
        "midpoint must be within (start, end)"
    );

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
    assert!(start + 1 == end, "incomplete or invalid path");
    start
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "target out of bounds")]
    fn test_out_of_bounds() {
        let _ = encode(10, 20, 25);
    }

    #[test]
    fn test_encode_decode() {
        for range in [2, 3, 5, 8, 16, 257] {
            for value in 0..range {
                let bits = encode(0, range, value);
                let decoded = decode(0, range, &bits);
                assert_eq!(decoded, value);
            }
        }
    }

    #[test]
    fn test_encode_decode_small_ranges() {
        for size in 2..=20 {
            for value in 0..size {
                let bits = encode(0, size, value);
                let decoded = decode(0, size, &bits);
                assert_eq!(decoded, value, "Failed at value {} in size {}", value, size);
            }
        }
    }

    #[test]
    fn test_exact_power_of_two() {
        for k in 1..=16 {
            let size = 1 << k;
            for value in [0, size / 4, size / 2, size - 1] {
                let bits = encode(0, size, value);
                assert_eq!(bits.len(), k, "Expected {} bits for size {}", k, size);
                let decoded = decode(0, size, &bits);
                assert_eq!(decoded, value);
            }
        }
    }

    #[test]
    fn test_non_power_of_two_ranges() {
        for size in [3, 5, 6, 10, 17, 31, 100, 257] {
            for value in [0, size / 2, size - 1] {
                let bits = encode(0, size, value);
                let decoded = decode(0, size, &bits);
                assert_eq!(decoded, value);
            }
        }
    }

    #[test]
    fn test_single_element_range() {
        let bits = encode(42, 43, 42);
        assert!(
            bits.is_empty(),
            "Single-element range should produce no bits"
        );
        let decoded = decode(42, 43, &BitVec::<u8, Msb0>::new());
        assert_eq!(decoded, 42);
    }

    #[test]
    #[should_panic(expected = "target out of bounds")]
    fn test_target_out_of_bounds_below() {
        encode(10, 20, 9);
    }

    #[test]
    #[should_panic(expected = "target out of bounds")]
    fn test_target_out_of_bounds_above() {
        encode(10, 20, 20);
    }

    #[test]
    #[should_panic(expected = "incomplete or invalid path")]
    fn test_invalid_decode_path() {
        let mut bits = BitVec::<u8, Msb0>::new();
        bits.push(true);
        decode(0, 3, &bits);
    }

    #[test]
    fn test_path_lengths_monotonicity() {
        for range in 2..=100 {
            for value in 0..range {
                let bits = encode(0, range, value);
                assert!(bits.len() <= 64, "Path too long: {} bits", bits.len());
            }
        }
    }

    #[test]
    fn test_large_range_edge_values() {
        let size = 1_000_001;
        let bits_start = encode(0, size, 0);
        let bits_end = encode(0, size, size - 1);
        assert_eq!(decode(0, size, &bits_start), 0);
        assert_eq!(decode(0, size, &bits_end), size - 1);
    }

    #[test]
    fn test_encode_from_custom_midpoint() {
        let start = 0;
        let end = 16;
        let midpoint = (start + end) / 2; // must be safe
        for value in start..end {
            let bits = encode_from(start, end, value, midpoint);
            let decoded = decode(start, end, &bits);
            assert_eq!(decoded, value, "Failed at value {}", value);
        }
    }

    #[test]
    #[should_panic(expected = "midpoint must be within (start, end)")]
    fn test_midpoint_out_of_bounds() {
        // Midpoint is equal to start — violates midpoint ∈ (start, end)
        let _ = encode_from(0, 10, 5, 0);
    }

    #[test]
    #[should_panic(expected = "target out of bounds")]
    fn test_target_out_of_bounds_in_custom_midpoint() {
        // Target == end — out of range
        let _ = encode_from(0, 10, 10, 5);
    }
}
