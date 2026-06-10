// 20. Valid Parentheses

// Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.

// An input string is valid if:
// Open brackets must be closed by the same type of brackets.
// Open brackets must be closed in the correct order.
// Every close bracket has a corresponding open bracket of the same type.

// Example 1:
// Input: s = "()"
// Output: true

// Example 2:
// Input: s = "()[]{}"
// Output: true

// Example 3:
// Input: s = "(]"
// Output: false

// Example 4:
// Input: s = "([])"
// Output: true

// Example 5:
// Input: s = "([)]"
// Output: false
struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut s_vec: Vec<char> = vec![]; // Это вектор который хранит скобки, если есть
        // последняя открывающая скобка того же типа что закрывающая , то они взаимноуничтожаются
        // если в конце цикла вектро пуст, значит выражение валидно.
        for bracket in s.chars() {
            if bracket == '(' || bracket == '{' || bracket == '[' {
                s_vec.push(bracket);
            }
        }

        for c in s_vec {
            print!("{:?}", c);
        }
        println!("");

        true
    }
}

fn main() {
    let test_str = "()".to_string();
    Solution::is_valid(test_str);
}
