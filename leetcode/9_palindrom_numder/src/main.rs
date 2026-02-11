// 9. Palindrome Number
// Given an integer x, return true if x is a palindrom, and false otherwise.

// Example 1:
// Input: x = 121
// Output: true
// Explanation: 121 reads as 121 from left to right and from right to left.

// Example 2:
// Input: x = -121
// Output: false
// Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.

// Example 3:
// Input: x = 10
// Output: false
// Explanation: Reads 01 from right to left. Therefore it is not a palindrome.

// Constraints:
//     -231 <= x <= 231 - 1

// Follow up: Could you solve it without converting the integer to a string?

struct Solution;

fn main() {
    println!("121 is palindrome: {}", Solution::is_palindrome(121));
    println!("123 is palindrome: {}", Solution::is_palindrome(123));
}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let mut ost;
        let mut cel = x;
        let mut my_vec: Vec<i32> = vec![];
        while cel > 0 {
            ost = cel % 10;
            cel = cel / 10;
            my_vec.push(ost);
            //println!("ost={}, cel={}", ost, cel);
        }
        //println!("my_vec={:?}", my_vec);
        let mut clone = my_vec.clone();
        clone.reverse();
        if my_vec == clone {
            return true;
        } else {
            return false;
        }
    }
}
