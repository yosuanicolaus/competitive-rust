//{"name":"advent-of-code-241210-a","group":"Advent of Code 241210","url":"https://adventofcode.com/2024/day/10","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"advent-of-code-241210-a"}}}

use std::collections::HashSet;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn get_trail_score(
    grid: &Vec<Vec<u32>>,
    curr_y: usize,
    curr_x: usize,
    seen_height: &mut HashSet<(usize, usize)>,
) -> u32 {
    if grid[curr_y][curr_x] == 9 {
        if seen_height.contains(&(curr_y, curr_x)) {
            return 0;
        } else {
            seen_height.insert((curr_y, curr_x));
            return 1;
        }
    }

    let target_height = grid[curr_y][curr_x] + 1;
    let mut score = 0;

    if curr_y > 0 && grid[curr_y - 1][curr_x] == target_height {
        score += get_trail_score(grid, curr_y - 1, curr_x, seen_height);
    }
    if curr_y < grid.len() - 1 && grid[curr_y + 1][curr_x] == target_height {
        score += get_trail_score(grid, curr_y + 1, curr_x, seen_height);
    }
    if curr_x > 0 && grid[curr_y][curr_x - 1] == target_height {
        score += get_trail_score(grid, curr_y, curr_x - 1, seen_height);
    }
    if curr_x < grid[0].len() - 1 && grid[curr_y][curr_x + 1] == target_height {
        score += get_trail_score(grid, curr_y, curr_x + 1, seen_height);
    }

    score
}

fn solve(input: &mut Input, output: &mut Output) {
    let mut grid: Vec<Vec<u32>> = vec![];

    while !input.is_empty() {
        let line = input.read_line();
        grid.push(line.chars().map(|c| c.to_digit(10).unwrap()).collect());
    }

    let mut ans = 0;

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == 0 {
                let mut seen_height: HashSet<(usize, usize)> = HashSet::new();
                ans += get_trail_score(&grid, y, x, &mut seen_height);
            }
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
