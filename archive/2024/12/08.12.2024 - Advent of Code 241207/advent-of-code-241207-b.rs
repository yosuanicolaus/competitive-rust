//{"name":"advent-of-code-241207-b","group":"Advent of Code 241207","url":"https://adventofcode.com/2024/day/7","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"advent-of-code-241207-b"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn can_calibrate(target: u128, nums: &Vec<u128>, try_idx: usize, curr_num: u128) -> bool {
    if target == curr_num && try_idx == nums.len() {
        return true;
    } else if target < curr_num {
        return false;
    } else if try_idx == nums.len() {
        // && curr_num <= target
        return false;
    }

    let parse_res = format!("{}{}", curr_num, nums[try_idx])
        .parse::<u128>()
        .unwrap();

    can_calibrate(target, nums, try_idx + 1, curr_num + nums[try_idx])
        || can_calibrate(target, nums, try_idx + 1, curr_num * nums[try_idx])
        || can_calibrate(target, nums, try_idx + 1, parse_res)
}

fn solve(input: &mut Input, output: &mut Output) {
    let mut ans = 0;

    while !input.is_empty() {
        let line = input.read_line();
        let (target_num, nums) = line
            .split_once(": ")
            .map(|(s1, s2)| {
                (
                    s1.parse::<u128>().unwrap(),
                    s2.split_whitespace()
                        .map(|s2n| s2n.parse::<u128>().unwrap())
                        .collect::<Vec<u128>>(),
                )
            })
            .unwrap();

        if can_calibrate(target_num, &nums, 1, nums[0]) {
            ans += target_num;
        }
    }

    output.print_line(ans);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    input.is_empty()
}

//START MAIN
mod tester;
fn main() {
    tester::run_tests();
}
//END MAIN
