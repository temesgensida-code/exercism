pub fn series(digits: &str, len: usize) -> Vec<String> {
    // Return empty vector immediately if len is 0 or larger than the input string
    if len == 0 || len > digits.len() {
        return Vec::new();
    }

    let mut outputs: Vec<String> = Vec::new();
    
    // Correct loop boundary to prevent underflow or out-of-bounds indexing
    for lett in 0..=(digits.len() - len) {
        let slice = &digits[lett..(lett + len)];
        outputs.push(slice.to_string());
    }
    
    outputs
}
