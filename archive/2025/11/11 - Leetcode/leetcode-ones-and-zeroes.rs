//{"name":"leetcode-ones-and-zeroes","group":"Leetcode","url":"https://leetcode.com/problems/ones-and-zeroes/description/","interactive":false,"timeLimit":2000,"tests":[],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"leetcode-ones-and-zeroes"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use std::cmp;

pub struct Solution {}

// Verdict: TLE
// maybe dp is not the right approach? ...
// at least 27/77 test case passed ...
// strs.len() can get to 600. 2^600 is way too large for DP here

fn dp(m: i32, n: i32, res: i32, pool: &Vec<(i32, i32)>, pool_idx: usize, max_res: &mut i32) {
    if pool_idx == pool.len() || (m == 0 && n == 0) {
        return;
    }
    let (cost_m, cost_n) = pool[pool_idx];
    let new_m = m - cost_m;
    let new_n = n - cost_n;
    let new_res = res + 1;

    if (new_m >= 0 && new_n >= 0) {
        // can take
        *max_res = cmp::max(*max_res, new_res);
        dp(new_m, new_n, new_res, pool, pool_idx + 1, max_res);
    }
    // not take
    dp(m, n, res, pool, pool_idx + 1, max_res);
}

impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let mut max_res = 0;
        let mut pool: Vec<(i32, i32)> = vec![];

        for digits in strs {
            let mut num0 = 0;
            let mut num1 = 0;
            for digit in digits.chars() {
                if digit == '0' {
                    num0 += 1;
                } else {
                    num1 += 1;
                }
            }
            pool.push((num0, num1));
        }

        dp(m, n, 0, &pool, 0, &mut max_res);
        max_res
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    while !input.is_exhausted() {
        let strs = input.read_leetcode_vec_str();
        let m = input.read_int();
        let n = input.read_int();
        let res = Solution::find_max_form(strs, m, n);
        output.println(res);
        output.flush();
    }
    input.is_empty()
}

//START MAIN
mod tester;
fn main() {
    tester::run_main();
}
//END MAIN
