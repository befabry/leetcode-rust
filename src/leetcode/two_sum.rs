use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();

    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;

        if let Some(&complement_index) = map.get(&complement) {
            return vec![complement_index as i32, i as i32];
        }

        map.insert(num, i);
    }

    vec![] // This line should never be reached due to the problem constraints
}
