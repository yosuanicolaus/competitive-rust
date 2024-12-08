//{"name":"advent-of-code-241208-a","group":"Advent of Code 241208","url":"https://adventofcode.com/2024/day/8","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"advent-of-code-241208-a"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use std::collections::HashMap;

fn within_grid_bounds(grid: &Vec<Vec<char>>, test_y: i32, test_x: i32) -> bool {
    test_y >= 0 && test_y < grid.len() as i32 && test_x >= 0 && test_x < grid[0].len() as i32
}

fn solve(input: &mut Input, output: &mut Output) {
    let mut antenna_grid: Vec<Vec<char>> = vec![];
    let mut antinode_grid: Vec<Vec<char>> = vec![];
    let mut ans = 0;

    while !input.is_empty() {
        let grid_line: Vec<char> = input.read_line().chars().collect();
        antinode_grid.push(vec!['.'; grid_line.len()]);
        antenna_grid.push(grid_line);
    }

    let mut antenna_locations: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for y in 0..antenna_grid.len() {
        for x in 0..antenna_grid[0].len() {
            if antenna_grid[y][x] != '.' {
                let locations = antenna_locations.entry(antenna_grid[y][x]).or_default();
                locations.push((y, x));
            }
        }
    }

    // fill antinode grid
    for (_, locations) in antenna_locations {
        for (cy, cx) in &locations {
            for (oy, ox) in &locations {
                if cy == oy && cx == ox {
                    continue;
                }
                let dy = *oy as i32 - *cy as i32;
                let dx = *ox as i32 - *cx as i32;
                let test_y = *cy as i32 - dy;
                let test_x = *cx as i32 - dx;

                if within_grid_bounds(&antinode_grid, test_y, test_x) {
                    if antinode_grid[test_y as usize][test_x as usize] != '#' {
                        antinode_grid[test_y as usize][test_x as usize] = '#';
                        ans += 1;
                    }
                }
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
