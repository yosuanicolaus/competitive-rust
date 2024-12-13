//{"name":"advent-of-code-241213-a","group":"Advent of Code 241213","url":"https://adventofcode.com/2024/day/13","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"advent-of-code-241213-a"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use regex::Regex;
use std::cmp;

fn solve(input: &mut Input, output: &mut Output) {
    let re_bta = Regex::new(r"Button A: X\+(?<bx>\d+), Y\+(?<by>\d+)").unwrap();
    let re_btb = Regex::new(r"Button B: X\+(?<bx>\d+), Y\+(?<by>\d+)").unwrap();
    let re_prize = Regex::new(r"Prize: X=(?<bx>\d+), Y=(?<by>\d+)").unwrap();
    let mut ans = 0;

    // let line = input.read_line();
    while !input.is_empty() {
        let (la, lb, lp) = (input.read_line(), input.read_line(), input.read_line());
        let caps_a = re_bta.captures(&la).unwrap();
        let caps_b = re_btb.captures(&lb).unwrap();
        let caps_p = re_prize.captures(&lp).unwrap();

        let cax = caps_a["bx"].parse::<i32>().unwrap();
        let cay = caps_a["by"].parse::<i32>().unwrap();
        let cbx = caps_b["bx"].parse::<i32>().unwrap();
        let cby = caps_b["by"].parse::<i32>().unwrap();
        let cpx = caps_p["bx"].parse::<i32>().unwrap();
        let cpy = caps_p["by"].parse::<i32>().unwrap();

        let mut best_cost = 1000;
        let mut found_cost = false;

        // naive bruteforce, because the limit is small (10_000 operation)
        for ta in 0..=100 {
            for tb in 0..=100 {
                let tx = ta * cax + tb * cbx;
                let ty = ta * cay + tb * cby;
                if tx == cpx && ty == cpy {
                    found_cost = true;
                    best_cost = cmp::min(best_cost, 3 * ta + tb);
                }
            }
        }
        if found_cost {
            ans += best_cost;
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
