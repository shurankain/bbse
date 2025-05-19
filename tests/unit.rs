use bbse::{decode, encode, encode_from};
use bitvec::order::Msb0;
use bitvec::vec::BitVec;

#[test]
#[should_panic(expected = "target (25) out of bounds [10, 20)")]
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
#[should_panic(expected = "target (9) out of bounds [10, 20)")]
fn test_target_out_of_bounds_below() {
    encode(10, 20, 9);
}

#[test]
#[should_panic(expected = "target (20) out of bounds [10, 20)")]
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
    let midpoint = (start + end) / 2;
    for value in start..end {
        let bits = encode_from(start, end, value, midpoint);
        let decoded = decode(start, end, &bits);
        assert_eq!(decoded, value, "Failed at value {}", value);
    }
}

#[test]
#[should_panic(expected = "midpoint (0) must be within (start=0, end=10)")]
fn test_midpoint_out_of_bounds() {
    let _ = encode_from(0, 10, 5, 0);
}

#[test]
#[should_panic(expected = "target (10) out of bounds [0, 10)")]
fn test_target_out_of_bounds_in_custom_midpoint() {
    let _ = encode_from(0, 10, 10, 5);
}
