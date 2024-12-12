//{"name":"advent-of-code-241212-a","group":"Advent of Code 241212","url":"https://adventofcode.com/2024/day/12","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"advent-of-code-241212-a"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn traverse(
    land: &Vec<Vec<char>>,
    y: usize,
    x: usize,
    visited: &mut Vec<Vec<bool>>,
    total_area: &mut i32,
    total_fence: &mut i32,
) {
    if visited[y][x] {
        return;
    }

    visited[y][x] = true;
    *total_area += 1;

    // add fence on all possible sides
    if y == 0 || land[y - 1][x] != land[y][x] {
        *total_fence += 1;
    }
    if y == land.len() - 1 || land[y + 1][x] != land[y][x] {
        *total_fence += 1;
    }
    if x == 0 || land[y][x - 1] != land[y][x] {
        *total_fence += 1;
    }
    if x == land[0].len() - 1 || land[y][x + 1] != land[y][x] {
        *total_fence += 1;
    }

    // traverse for all possible neighbors
    if y > 0 && land[y - 1][x] == land[y][x] && !visited[y - 1][x] {
        traverse(land, y - 1, x, visited, total_area, total_fence);
    }
    if y + 1 < land.len() && land[y + 1][x] == land[y][x] && !visited[y + 1][x] {
        traverse(land, y + 1, x, visited, total_area, total_fence);
    }
    if x > 0 && land[y][x - 1] == land[y][x] && !visited[y][x - 1] {
        traverse(land, y, x - 1, visited, total_area, total_fence);
    }
    if x + 1 < land[0].len() && land[y][x + 1] == land[y][x] && !visited[y][x + 1] {
        traverse(land, y, x + 1, visited, total_area, total_fence);
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
