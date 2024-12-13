//{"name":"advent-of-code-241213-b","group":"Advent of Code 241213","url":"https://adventofcode.com/2024/day/13","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"advent-of-code-241213-b"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use regex::Regex;

fn solve(input: &mut Input, output: &mut Output) {
    let re_bta = Regex::new(r"Button A: X\+(?<bx>\d+), Y\+(?<by>\d+)").unwrap();
    let re_btb = Regex::new(r"Button B: X\+(?<bx>\d+), Y\+(?<by>\d+)").unwrap();
    let re_prize = Regex::new(r"Prize: X=(?<bx>\d+), Y=(?<by>\d+)").unwrap();
    let mut ans = 0;

    while !input.is_empty() {
        let (la, lb, lp) = (input.read_line(), input.read_line(), input.read_line());
        let caps_a = re_bta.captures(&la).unwrap();
        let caps_b = re_btb.captures(&lb).unwrap();
        let caps_p = re_prize.captures(&lp).unwrap();

        let xa = caps_a["bx"].parse::<i128>().unwrap();
        let ya = caps_a["by"].parse::<i128>().unwrap();
        let xb = caps_b["bx"].parse::<i128>().unwrap();
        let yb = caps_b["by"].parse::<i128>().unwrap();
        let xp = caps_p["bx"].parse::<i128>().unwrap() + 10000000000000;
        let yp = caps_p["by"].parse::<i128>().unwrap() + 10000000000000;

        // Use Cramer's Rule for 2 equations with 2 unknowns
        let d = xa * yb - ya * xb;
        let a = (yb * xp - xb * yp) / d;
        let b = (xa * yp - ya * xp) / d;

        if a * xa + b * xb == xp && a * ya + b * yb == yp {
            ans += a * 3 + b;
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
