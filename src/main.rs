mod is_palindrome;
mod longest_common_prefix;
mod prefixes_div_by5;
mod roman_to_int;
mod two_sum;

use is_palindrome::is_palindrome;
use longest_common_prefix::longest_common_prefix;
use prefixes_div_by5::prefixes_div_by5;
use roman_to_int::roman_to_int;
use two_sum::two_sum;

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
}
