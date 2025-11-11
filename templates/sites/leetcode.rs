use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

pub struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> i32 {
        0
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    while !input.is_exhausted() {
        let nums = input.read_leetcode_vec_int();
        let target = input.read_int();
        let res = Solution::two_sum(nums, target);
        output.println(res);
        output.flush();
    }
    input.is_empty()
}
