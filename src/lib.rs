//! # bbse — Backward Binary Search Encoding
//!
//! A minimal and deterministic encoding scheme for any sorted range,
//! based on walking the binary search path to a target value.
//!
//! Useful in scenarios requiring:
//! - compact, prefix-free representation of integers from a range;
//! - simple and reversible encoding for sorted domains;
//! - constant-time decoding without full table reconstruction (unlike Elias/Fano).
//!
//! ## Example
//! ```
//! use bbse::{encode, decode};
//! let path = bbse::encode(0, 16, 5);       // binary search path to 5 in [0..16)
//! let value = bbse::decode(0, 16, &path);  // == 5
//! assert_eq!(value, 5);
//! ```

use bitvec::order::Msb0;
use bitvec::vec::BitVec;

/// Encodes a value as a binary decision sequence (left/right),
/// given a sorted range [`start`, `end`) and the target value.
///
/// Returns a `BitVec` of left (0) and right (1) decisions.
///
/// # Panics
/// Panics if `target` is not in the range.
///
/// # Example
/// ```
/// let path = bbse::encode(0, 8, 6);
/// ```
pub fn encode(mut start: usize, mut end: usize, target: usize) -> BitVec<u8, Msb0> {
    assert!(start <= target && target < end, "target out of bounds");

    let mut path = BitVec::<u8, Msb0>::new();
    while end - start > 1 {
        let mid = (start + end) / 2;
        if target < mid {
            path.push(false); // go left
            end = mid;
        } else {
            path.push(true); // go right
            start = mid;
        }
    }
    path
}

/// Decodes a binary decision sequence into the value it encodes,
/// given the original [`start`, `end`) range.
///
/// # Panics
/// Panics if the path is invalid or incomplete.
///
/// # Example
/// ```
/// let value = bbse::decode(0, 8, &bbse::encode(0, 8, 6));
/// assert_eq!(value, 6);
/// ```
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
        bits.push(true); // leads to [1, 3), not yet resolved
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
}
