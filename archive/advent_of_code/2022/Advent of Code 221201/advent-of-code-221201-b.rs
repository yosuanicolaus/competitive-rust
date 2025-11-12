//{"name":"advent-of-code-221201-b","group":"Advent of Code 221201","url":"https://adventofcode.com/2022/day/1","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"advent-of-code-221201-b"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

// Can also use heap/priority queue.

fn solve(input: &mut Input, out: &mut Output) {
    let lines = input.read_raw();
    let mut lines: Vec<&str> = lines.split('\n').collect();
    let mut best_calories = [0, 0, 0];
    let mut cur_calories = 0;

    if !lines.last().unwrap().is_empty() {
        lines.push("");
    }

    for digits in lines {
        if digits == "" {
            if cur_calories > best_calories[0] {
                best_calories[0] = cur_calories;
            }
            cur_calories = 0;
            best_calories.sort();
        } else {
            cur_calories += digits.parse::<i32>().unwrap();
        }
    }

    out.println(best_calories.iter().sum::<i32>());
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
