use bbse::{decode, encode, encode_from, BBSEStack};

#[test]
fn test_basic_encode_decode() {
    for value in 0..8 {
        let path = encode(0, 8, value);
        let decoded = decode(0, 8, &path);
        assert_eq!(decoded, value);
    }
}

#[test]
fn test_stack_encoding_and_decoding() {
    let mut stack = BBSEStack::new();

    for value in 0..8 {
        stack.push(encode(0, 8, value));
    }

    let decoded = stack.decode_all(0, 8);
    assert_eq!(decoded, vec![0, 1, 2, 3, 4, 5, 6, 7]);
}

#[test]
fn test_single_element_range() {
    let path = encode(42, 43, 42);
    assert!(path.is_empty());
    let decoded = decode(42, 43, &path);
    assert_eq!(decoded, 42);
}

#[test]
#[should_panic(expected = "target (5) out of bounds [0, 5)")]
fn test_target_out_of_bounds_high() {
    let _ = encode(0, 5, 5);
}

#[test]
#[should_panic(expected = "target (0) out of bounds [1, 5)")]
fn test_target_out_of_bounds_low() {
    let _ = encode(1, 5, 0);
}

#[test]
#[should_panic(expected = "Invalid range: start (10) >= end (10)")]
fn test_empty_range() {
    let _ = encode(10, 10, 10);
}

#[test]
fn test_custom_midpoint_wrapper() {
    let path = encode_from(0, 8, 4, 4);
    let decoded = decode(0, 8, &path);
    assert_eq!(decoded, 4);
}

#[test]
#[should_panic(expected = "midpoint (0) must be within (start=0, end=10)")]
fn test_invalid_custom_midpoint() {
    let _ = encode_from(0, 10, 5, 0);
}
