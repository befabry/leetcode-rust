pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return String::new();
    }

    let mut result = String::new();
    let first_string = &strs[0];

    // Compare character by character across all strings
    for (i, char_to_compare) in first_string.chars().enumerate() {
        for s in &strs[1..] {
            // If this string is shorter or has different character at position i
            if let Some(current_char) = s.chars().nth(i) {
                if current_char != char_to_compare {
                    return result;
                }
            } else {
                // Current string is shorter than first string
                return result;
            }
        }
        // All strings have the same character at position i
        result.push(char_to_compare);
    }

    result
}
