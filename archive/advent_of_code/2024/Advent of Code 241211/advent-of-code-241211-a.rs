//{"name":"advent-of-code-241211-a","group":"Advent of Code 241211","url":"https://adventofcode.com/2024/day/11","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"advent-of-code-241211-a"}}}

// naive bruteforce solution; doesn't work on part-b, 75 iteration

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

const ITERATION: usize = 25;

fn split_to_two(s: &str) -> String {
    let (s1, s2) = s.split_at(s.len() / 2);
    let ss1 = s1.parse::<i64>().unwrap().to_string();
    let ss2 = s2.parse::<i64>().unwrap().to_string();
    format!("{} {}", ss1.as_str(), ss2.as_str())
}

fn solve(input: &mut Input, output: &mut Output) {
    let mut digits: Vec<String> = input
        .read_line()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    for _ in 0..ITERATION {
        for s in &mut digits {
            if s == "0" {
                *s = "1".to_string();
            } else if s.len() % 2 == 0 {
                *s = split_to_two(s);
            } else {
                *s = (s.parse::<i64>().unwrap() * 2024).to_string();
            }
        }
        let sum_s: String = digits.join(" ");
        digits.clear();
        for new_s in sum_s.split_whitespace() {
            digits.push(new_s.to_string());
        }
    }

    output.print_line(digits.len());
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
