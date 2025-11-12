//{"name":"advent-of-code-221201-a","group":"Advent of Code 221201","url":"https://adventofcode.com/2022/day/1","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"advent-of-code-221201-a"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output) {
    let lines = input.read_raw();
    let mut lines: Vec<&str> = lines.split('\n').collect();
    let mut max_calories = 0;
    let mut cur_calories = 0;

    if !lines.last().unwrap().is_empty() {
        lines.push("");
    }

    for digits in lines {
        if digits == "" {
            max_calories = std::cmp::max(max_calories, cur_calories);
            cur_calories = 0;
        } else {
            cur_calories += digits.parse::<i32>().unwrap();
        }
    }

    out.println(max_calories);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    input.is_empty()
}

//START MAIN
mod tester;
fn main() {
    tester::run_main();
}
//END MAIN
