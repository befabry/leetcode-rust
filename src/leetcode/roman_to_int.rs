pub fn roman_to_int(s: String) -> i32 {
    let mut res = 0;
    let mut prev_value = 0;

    for ch in s.chars().rev() {
        let current_value = match ch {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        };

        if current_value < prev_value {
            res -= current_value;
        } else {
            res += current_value;
        }

        prev_value = current_value;
    }

    res
}
