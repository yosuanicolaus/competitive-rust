//{"name":"advent-of-code-221202-a","group":"Advent of Code 221202","url":"https://adventofcode.com/2022/day/2","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"advent-of-code-221202-a"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use std::collections::HashMap;

fn solve(input: &mut Input, out: &mut Output) {
    let score_map = HashMap::from([
        ('X', HashMap::from([('A', 4), ('B', 1), ('C', 7)])),
        ('Y', HashMap::from([('A', 8), ('B', 5), ('C', 2)])),
        ('Z', HashMap::from([('A', 3), ('B', 9), ('C', 6)])),
    ]);
    let mut total_score = 0;

    while !input.is_empty() {
        let elf_move = input.read_char();
        let my_move = input.read_char();
        total_score += score_map[&my_move][&elf_move];
    }
    out.println(total_score);
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
