pub fn is_parenthesis_valid(s: String) -> bool {
    let mut stack = Vec::new();
    for i in s.chars() {
        match i {
            '{' => stack.push('}'),
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            _ => {
                if Some(i) != stack.pop() {
                    return false;
                }
            }
        }
    }
    stack.is_empty()
}
