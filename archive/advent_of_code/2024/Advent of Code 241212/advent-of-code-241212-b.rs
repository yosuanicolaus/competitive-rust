//{"name":"advent-of-code-241212-b","group":"Advent of Code 241212","url":"https://adventofcode.com/2024/day/12","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"advent-of-code-241212-b"}}}

use std::collections::VecDeque;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn traverse(
    land: &[Vec<char>],
    start_y: usize,
    start_x: usize,
    visited: &mut [Vec<bool>],
    total_area: &mut i32,
    total_fence: &mut i32,
) {
    let curr_land = land[start_y][start_x];
    let mut queue: VecDeque<(usize, usize)> = VecDeque::from([(start_y, start_x)]);

    while !queue.is_empty() {
        let (y, x) = queue.pop_front().unwrap();
        if visited[y][x] {
            continue;
        }

        visited[y][x] = true;
        *total_area += 1;

        // add fence everytime a corner is detected
        // TODO: fence calculation for part 2, real test input is wrong :(
        let nb_n = y > 0 && land[y - 1][x] == curr_land;
        let nb_e = x + 1 < land[0].len() && land[y][x + 1] == curr_land;
        let nb_s = y + 1 < land.len() && land[y + 1][x] == curr_land;
        let nb_w = x > 0 && land[y][x - 1] == curr_land;

        let nb_ne = y > 0 && x + 1 < land[0].len() && land[y - 1][x + 1] == curr_land;
        let nb_se = y + 1 < land.len() && x + 1 < land[0].len() && land[y + 1][x + 1] == curr_land;
        let nb_sw = y + 1 < land.len() && x > 0 && land[y + 1][x - 1] == curr_land;
        let nb_nw = y > 0 && x > 0 && land[y - 1][x - 1] == curr_land;

        if !nb_ne && (nb_n == nb_e) {
            *total_fence += 1;
        }
        if !nb_se && (nb_s == nb_e) {
            *total_fence += 1;
        }
        if !nb_sw && (nb_s == nb_w) {
            *total_fence += 1;
        }
        if !nb_nw && (nb_n == nb_w) {
            *total_fence += 1;
        }

        // traverse all possible neighbors
        if y > 0 && land[y - 1][x] == curr_land && !visited[y - 1][x] {
            queue.push_back((y - 1, x));
        }
        if y + 1 < land.len() && land[y + 1][x] == curr_land && !visited[y + 1][x] {
            queue.push_back((y + 1, x));
        }
        if x > 0 && land[y][x - 1] == curr_land && !visited[y][x - 1] {
            queue.push_back((y, x - 1));
        }
        if x + 1 < land[0].len() && land[y][x + 1] == curr_land && !visited[y][x + 1] {
            queue.push_back((y, x + 1));
        }
    }
}

fn solve(input: &mut Input, output: &mut Output) {
    let mut land: Vec<Vec<char>> = vec![];
    let mut visited: Vec<Vec<bool>> = vec![];

    while !input.is_empty() {
        let line_vec: Vec<char> = input.read_line().chars().collect();
        visited.push(vec![false; line_vec.len()]);
        land.push(line_vec);
    }

    let mut ans = 0;

    for y in 0..land.len() {
        for x in 0..land[0].len() {
            if !visited[y][x] {
                let mut total_area = 0;
                let mut total_fence = 0;
                traverse(&land, y, x, &mut visited, &mut total_area, &mut total_fence);
                ans += total_area * total_fence;
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
