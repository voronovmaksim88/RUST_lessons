fn main() {
    let solution = Solution::roman_to_int("IV".to_string());
    println!("итог {}", solution);
}

struct Solution {}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut n = 0;
        let mut prev_l: char = ' ';
        for l in s.chars().rev() {
            match l {
                'I' => {
                    if prev_l == 'V' || prev_l == 'X' {
                        n -= 1;
                    } else {
                        n += 1;
                    }
                }
                'V' => n += 5,
                'X' => {
                    if prev_l == 'L' || prev_l == 'C' {
                        n -= 10;
                    } else {
                        n += 10;
                    }
                }
                'L' => n += 50,
                'C' => {
                    if prev_l == 'D' || prev_l == 'M' {
                        n -= 100;
                    } else {
                        n += 100;
                    }
                }
                'D' => n += 500,
                'M' => n += 1000,
                _ => {}
            }
            // println!("{}", n);
            prev_l = l;
        }
        n
    }
}
