pub fn count_partitions(nums: Vec<i32>) -> i32 {
    let mut count = 0;
    for i in 1..nums.len() {
        let left_sum: i32 = nums[0..i].iter().sum();
        let right_sum: i32 = nums[i..].iter().sum();

        if (left_sum - right_sum) % 2 == 0 {
            count += 1;
        }

        // println!("i = {}: left {:?}, right {:?}", i, left_sum, right_sum);
    }
    count
}
