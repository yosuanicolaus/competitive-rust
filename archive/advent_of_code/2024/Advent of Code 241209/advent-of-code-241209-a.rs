//{"name":"advent-of-code-241209-a","group":"Advent of Code 241209","url":"https://adventofcode.com/2024/day/9","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"advent-of-code-241209-a"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn solve(input: &mut Input, output: &mut Output) {
    let line = input.read_line();
    let mut memory: Vec<i32> = vec![];
    let mut is_block = true;
    let mut curr_id = 0;

    for ch in line.chars() {
        let digit = ch.to_digit(10).unwrap();
        if is_block {
            memory.append(&mut vec![curr_id; digit as usize]);
            curr_id += 1;
        } else {
            memory.append(&mut vec![-1; digit as usize]);
        }
        is_block = !is_block;
    }

    let mut a: usize = 0;
    let mut b: usize = memory.len() - 1;

    while a < b {
        if memory[a] != -1 {
            a += 1;
        } else if memory[b] == -1 {
            b -= 1;
        } else {
            (memory[a], memory[b]) = (memory[b], memory[a]);
            a += 1;
            b -= 1;
        }
    }

    let mut ans: i64 = 0;
    for i in 1..memory.len() {
        if memory[i] == -1 {
            break;
        }
        ans += i as i64 * memory[i] as i64;
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
