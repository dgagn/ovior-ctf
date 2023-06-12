pub fn parse_signature(sig: &str) -> Vec<Option<u8>> {
    sig.as_bytes()
        .chunks(2)
        .map(|chunk| {
            let s = std::str::from_utf8(chunk).unwrap();
            if s == "??" {
                None
            } else {
                Some(u8::from_str_radix(s, 16).unwrap())
            }
        })
        .collect()
}

pub fn find_pattern_start(input: &[u8], pattern: &[Option<u8>]) -> Option<usize> {
    input.windows(pattern.len()).position(|window| {
        window
            .iter()
            .zip(pattern)
            .all(|(byte, pattern_byte)| pattern_byte.map_or(true, |b| b == *byte))
    })
}

#[test]
fn test_parse_signature() {
    assert_eq!(parse_signature("00??ff"), vec![Some(0), None, Some(255)]);
    assert_eq!(parse_signature("??????"), vec![None, None, None]);
    assert_eq!(parse_signature("ab"), vec![Some(171)]);
}

#[test]
fn test_find_pattern_start() {
    let input = b"hello world";
    let pattern = vec![Some(104), Some(101), Some(108), Some(108), Some(111)];
    assert_eq!(find_pattern_start(input, &pattern), Some(0));

    let pattern = vec![Some(119), Some(111), Some(114), Some(108), Some(100)];
    assert_eq!(find_pattern_start(input, &pattern), Some(6));

    let pattern = vec![Some(108), None, None];
    assert_eq!(find_pattern_start(input, &pattern), Some(2));

    let pattern = vec![Some(108), None, Some(108)];
    assert_eq!(find_pattern_start(input, &pattern), None);
}
