//{"name":"advent-of-code-241212-b","group":"Advent of Code 241212","url":"https://adventofcode.com/2024/day/12","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"advent-of-code-241212-b"}}}

use std::collections::VecDeque;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn bfs_cost(land: &[Vec<char>], start_y: usize, start_x: usize, visited: &mut [Vec<bool>]) -> i32 {
    let curr_land = land[start_y][start_x];
    let mut queue: VecDeque<(usize, usize)> = VecDeque::from([(start_y, start_x)]);
    let mut total_area = 0;
    let mut total_fence = 0;

    while !queue.is_empty() {
        let (y, x) = queue.pop_front().unwrap();
        if visited[y][x] {
            continue;
        }

        visited[y][x] = true;
        total_area += 1;

        // add fence everytime a corner is detected
        let nb_n = y > 0 && land[y - 1][x] == curr_land;
        let nb_e = x + 1 < land[0].len() && land[y][x + 1] == curr_land;
        let nb_s = y + 1 < land.len() && land[y + 1][x] == curr_land;
        let nb_w = x > 0 && land[y][x - 1] == curr_land;

        let nb_ne = y > 0 && x + 1 < land[0].len() && land[y - 1][x + 1] == curr_land;
        let nb_se = y + 1 < land.len() && x + 1 < land[0].len() && land[y + 1][x + 1] == curr_land;
        let nb_sw = y + 1 < land.len() && x > 0 && land[y + 1][x - 1] == curr_land;
        let nb_nw = y > 0 && x > 0 && land[y - 1][x - 1] == curr_land;

        if (!nb_n && !nb_e) || (nb_n && nb_e && !nb_ne) {
            total_fence += 1;
        }
        if (!nb_s && !nb_e) || (nb_s && nb_e && !nb_se) {
            total_fence += 1;
        }
        if (!nb_s && !nb_w) || (nb_s && nb_w && !nb_sw) {
            total_fence += 1;
        }
        if (!nb_n && !nb_w) || (nb_n && nb_w && !nb_nw) {
            total_fence += 1;
        }

        // traverse all possible neighbors
        if y > 0 && land[y - 1][x] == curr_land {
            queue.push_back((y - 1, x));
        }
        if y + 1 < land.len() && land[y + 1][x] == curr_land {
            queue.push_back((y + 1, x));
        }
        if x > 0 && land[y][x - 1] == curr_land {
            queue.push_back((y, x - 1));
        }
        if x + 1 < land[0].len() && land[y][x + 1] == curr_land {
            queue.push_back((y, x + 1));
        }
    }

    total_area * total_fence
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
                ans += bfs_cost(&land, y, x, &mut visited);
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
