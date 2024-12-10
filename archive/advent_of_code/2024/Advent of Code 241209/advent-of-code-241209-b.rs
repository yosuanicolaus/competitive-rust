//{"name":"advent-of-code-241209-b","group":"Advent of Code 241209","url":"https://adventofcode.com/2024/day/9","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"advent-of-code-241209-b"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use std::collections::{HashMap, HashSet};

fn solve(input: &mut Input, output: &mut Output) {
    let line = input.read_line();
    let mut compressed: Vec<usize> = vec![];

    for ch in line.chars() {
        let digit = ch.to_digit(10).unwrap() as usize;
        compressed.push(digit);
    }

    /* format: <compressed's free space idx>: [(moved block id, moved block size), ...] */
    let mut to_insert: HashMap<usize, Vec<(i32, usize)>> = HashMap::new();
    let mut to_neutralize: HashSet<usize> = HashSet::new();

    for i in (0..compressed.len()).rev().step_by(2) {
        let mem_size = compressed[i]; // is a memory block
        let block_id = (i / 2) as i32;

        // search for a free space
        for j in (1..i).step_by(2) {
            if compressed[j] >= mem_size {
                compressed[j] -= mem_size;
                to_insert.entry(j).or_default().push((block_id, mem_size));
                to_neutralize.insert(i);
                break;
            }
        }
    }

    let mut memory: Vec<i32> = vec![];

    // expand compressed to memory
    for i in 0..compressed.len() {
        if i % 2 == 0 {
            // is a memory block
            if to_neutralize.contains(&i) {
                memory.append(&mut vec![0; compressed[i]]);
            } else {
                let block_id = (i / 2) as i32;
                memory.append(&mut vec![block_id; compressed[i]]);
            }
        } else {
            // is a space block; which may have been booked with a block id to insert
            if let Some(insert_vec) = to_insert.get(&i) {
                for (block_id, block_size) in insert_vec {
                    memory.append(&mut vec![*block_id; *block_size]);
                }
            }
            memory.append(&mut vec![0; compressed[i]]);
        }
    }

    let mut ans: i64 = 0;
    for i in 0..memory.len() {
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
