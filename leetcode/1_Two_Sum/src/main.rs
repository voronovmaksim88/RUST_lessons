struct Solution;

fn main() {
    println!("{:?}", Solution::two_sum(vec![2, 7, 11, 15], 9));
}

impl Solution {
    // impl` в Rust — это блок реализации: в нём ты описываешь методы для типа (структуры, enum и т.д.).
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                if nums[i] + nums[j] == target {
                    return vec![i as i32, j as i32];
                }
            }
        }
        return vec![];
    }
}

// для запуска
// rustc main.rs
// .\main.exe

// мне пока непонятное, но очень быстрое о скорости решение
// use std::collections::HashMap;

// impl Solution {
//     pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
//         let mut snums:HashMap<&i32,usize> = HashMap::new();
//         for (i,elem) in nums.iter().enumerate() {
//             snums.insert(elem, i);
//         }

//         for (i,elem) in nums.iter().enumerate() {
//             match snums.get(&(target - elem)) {
//                 Some(&first_index) => {
//                     if first_index != i {
//                         return vec![first_index as i32, i as i32]
//                         }
//                     },
//                 None => (),
//             }
//         }
//     return vec![-1];
//     }
// }
