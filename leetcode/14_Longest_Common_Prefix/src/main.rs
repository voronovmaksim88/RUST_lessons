struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut end = false;
        let mut num = 0;
        let mut out_str = String::new();
        let mut cur_lets = vec![]; // текущий вектор букв, например все первые
        let mut cur_let: char; // текущий символ в цикле

        while !end {
            cur_lets = vec![];
            for str in &strs {
                //println!("{}", str.chars().nth(num).unwrap_or_default());
                cur_let = str.chars().nth(num).unwrap_or_default();
                if cur_let == '\0' {
                    end = true;
                    break;
                } else {
                    cur_lets.push(cur_let);
                }
            }
            if !end {
                //println!("{:?}", cur_lets);
                if cur_lets.iter().all(|&c| c == cur_lets[0]) {
                    out_str.push(cur_lets[0]);
                } else {
                    break;
                }
            }
            num += 1;
        }

        return out_str;
    }
}

fn main() {
    let my_sol = Solution::longest_common_prefix(vec![
        "flere".to_string(),
        "flow".to_string(),
        "flight".to_string(),
    ]);
    println!("");
    println!("{}", my_sol);
}
