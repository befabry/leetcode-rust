mod leetcode;

use leetcode::*;

fn main() {
    println!("{:?}", two_sum(vec![2, 7, 11, 15], 9)); // [0,1]
    println!("{:?}", two_sum(vec![3, 2, 4], 6)); // [1,2]
    println!("{:?}", two_sum(vec![3, 3], 6)); // [0,1]

    println!("{:?}", prefixes_div_by5(vec![0, 1, 1])); // [true, false, false]
    println!("{:?}", prefixes_div_by5(vec![1, 1, 1])); // [false, false, false]
    println!("{:?}", prefixes_div_by5(vec![0, 1, 1, 1, 1, 1])); // [true, false, false, false, true, false]

    println!("{:?}", is_palindrome(12121)); // true
    println!("{:?}", is_palindrome(123)); // false

    println!("{:?}", roman_to_int(String::from("MCCLXXV"))); // 1275
    println!("{:?}", roman_to_int(String::from("CCCLXXV"))); // 375
    println!("{:?}", roman_to_int(String::from("LVIII"))); // 58

    println!(
        "{:?}",
        longest_common_prefix(vec![
            String::from("flower"),
            String::from("flow"),
            String::from("flor")
        ])
    ); // flo

    println!("{:?}", is_parenthesis_valid(String::from("([)]")));

    let list1 = Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode { val: 3, next: None })),
        })),
    });

    let list2 = Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 3,
            next: Some(Box::new(ListNode { val: 4, next: None })),
        })),
    });

    println!("{:?}", merge_two_lists(Some(list1), Some(list2)));

    println!("{:?}", remove_duplicates(&mut vec![1, 1, 2])); // 2

    println!("{:?}", count_partitions(vec![10, 10, 3, 7, 6]));
}
