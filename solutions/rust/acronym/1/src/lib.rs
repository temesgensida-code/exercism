pub fn abbreviate(phrase: &str) -> String {
    phrase
        // Split on whitespace or hyphens
        .split(|c: char| c.is_whitespace() || c == '-')
        // Process each segment
        .flat_map(|word| {
            // Trim leading/trailing punctuation (like underscores, quotes, etc.)
            let clean_word = word.trim_matches(|c: char| !c.is_alphanumeric());
            let has_interior_caps = clean_word
                .chars()
                .skip(1)
                .any(|c| c.is_uppercase() && clean_word.chars().any(|ch| ch.is_lowercase()));

            clean_word
                .chars()
                .enumerate()
                .filter_map(move |(i, c)| {
                    if i == 0 && c.is_alphanumeric() {
                        Some(c)
                    } else if has_interior_caps && c.is_uppercase() {
                        // Takes 'T' out of 'HyperText'
                        Some(c)
                    } else {
                        None
                    }
                })
        })
        // Convert extracted letters to uppercase
        .flat_map(|c| c.to_uppercase())
        .collect()
}