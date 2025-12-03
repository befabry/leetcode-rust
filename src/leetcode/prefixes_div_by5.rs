pub fn prefixes_div_by5(nums: Vec<i32>) -> Vec<bool> {
    let mut result = vec![];
    let mut current = 0;

    for num in nums {
        current = (current * 2 + num) % 5;
        result.push(current == 0);
    }

    result
}
