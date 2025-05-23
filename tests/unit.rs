use bbse::{decode, decode_from, encode, encode_from, BBSEStack};

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

#[test]
fn test_custom_midpoint_encode_decode() {
    for target in 0..256 {
        let path = encode_from(0, 256, target, 64);
        let decoded = decode_from(0, 256, &path, 64);
        assert_eq!(
            decoded, target,
            "Custom midpoint failed on target={}",
            target
        );
    }
}

#[test]
fn test_custom_midpoint_small_range() {
    for target in 0..8 {
        let path = encode_from(0, 8, target, 3);
        let decoded = decode_from(0, 8, &path, 3);
        assert_eq!(decoded, target, "Failed on value {}", target);
    }
}

#[test]
fn test_custom_midpoint_unbalanced_left() {
    for target in 0..32 {
        let path = encode_from(0, 256, target, 8);
        let decoded = decode_from(0, 256, &path, 8);
        assert_eq!(decoded, target, "Left-side failure on {}", target);
    }
}

#[test]
fn test_custom_midpoint_unbalanced_right() {
    for target in 224..256 {
        let path = encode_from(0, 256, target, 240);
        let decoded = decode_from(0, 256, &path, 240);
        assert_eq!(decoded, target, "Right-side failure on {}", target);
    }
}

#[test]
fn test_custom_midpoint_center_precision() {
    let range = 0..256;
    let midpoints = [32, 64, 128, 192];

    for &mid in &midpoints {
        let path = encode_from(range.start, range.end, mid, mid);
        let decoded = decode_from(range.start, range.end, &path, mid);
        assert_eq!(decoded, mid, "Failed midpoint identity at {}", mid);
        assert!(
            path.is_empty(),
            "Expected empty path for direct midpoint match"
        );
    }
}
