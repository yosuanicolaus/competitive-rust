//{"name":"advent-of-code-221203-a","group":"Advent of Code 221203","url":"https://adventofcode.com/2022/day/3","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"advent-of-code-221203-a"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, out: &mut Output) {
    let mut total = 0;
    while !input.is_empty() {
        let line = input.read_line();
        let (comp1, comp2) = line.split_at(line.len() / 2);
        let mut comp1_bits = [false; 53];
        let mut comp2_bits = [false; 53];

        for (comp, comp_bits) in [(comp1, &mut comp1_bits), (comp2, &mut comp2_bits)] {
            for ch in comp.chars() {
                if ch.is_uppercase() {
                    comp_bits[ch as usize - 64 + 26] = true;
                } else {
                    comp_bits[ch as usize - 96] = true;
                }
            }
        }

        for i in (1..=52).rev() {
            if comp1_bits[i] && comp2_bits[i] {
                total += i as i32;
                break;
            }
        }
    }
    out.println(total);
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
