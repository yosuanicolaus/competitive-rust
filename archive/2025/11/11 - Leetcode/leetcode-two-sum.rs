//{"name":"leetcode-two-sum","group":"Leetcode","url":"https://leetcode.com/problems/two-sum/description/","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"leetcode-two-sum"}}}

use std::collections::HashMap;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

pub struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut seen_nums: HashMap<i32, i32> = HashMap::new();

        for (i, num) in nums.iter().enumerate() {
            let required = target - num;
            if seen_nums.contains_key(&required) {
                return vec![seen_nums[&required], i as i32];
            }
            seen_nums.insert(*num, i as i32);
        }
        panic!("no two sum found");
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let n = input.read_size();
    for _ in 0..n {
        let nums = input.read_leetcode_vec_int();
        let target = input.read_int();
        let res = Solution::two_sum(nums, target);
        output.println(res);
    }
    output.flush();
    input.is_empty()
}

//START MAIN
mod tester;
fn main() {
    tester::run_main();
}
//END MAIN
