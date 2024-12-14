//{"name":"advent-of-code-241214-a","group":"Advent of Code 241214","url":"https://adventofcode.com/2024/day/14","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"advent-of-code-241214-a"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use regex::Regex;

fn get_dead_coord(coord: usize) -> Option<usize> {
    if coord % 2 == 1 {
        Some(coord / 2)
    } else {
        None
    }
}

fn get_quadrant(
    sy: usize,
    sx: usize,
    dy: Option<usize>,
    dx: Option<usize>,
    npy: usize,
    npx: usize,
) -> Option<usize> {
    if dy.is_some() && npy == dy.unwrap() || dx.is_some() && npx == dx.unwrap() {
        return None;
    }

    let hy = sy / 2;
    let hx = sx / 2;

    if npy < hy {
        // upper half
        if npx < hx {
            // left half
            Some(0)
        } else {
            // right half
            Some(1)
        }
    } else {
        // bottom half
        if npx < hx {
            // left half
            Some(2)
        } else {
            // right half
            Some(3)
        }
    }
}

fn solve(input: &mut Input, out: &mut Output) {
    let re = Regex::new(r"p=(?<px>\-?\d+),(?<py>\-?\d+) v=(?<vx>\-?\d+),(?<vy>\-?\d+)")
        .expect("regex invalid");
    let sy = 103usize;
    let sx = 101usize;
    let dy = get_dead_coord(sy);
    let dx = get_dead_coord(sx);
    let mut quadrants = [0; 4];

    while !input.is_empty() {
        let inpline = input.read_line();
        let caps = re.captures(&inpline).expect("caps invalid");

        let py = caps["py"].parse::<i32>().unwrap();
        let px = caps["px"].parse::<i32>().unwrap();
        let vy = caps["vy"].parse::<i32>().unwrap();
        let vx = caps["vx"].parse::<i32>().unwrap();

        // calculate new py/px after 100 iteration
        let npy = (py + 100 * vy).rem_euclid(sy as i32) as usize;
        let npx = (px + 100 * vx).rem_euclid(sx as i32) as usize;

        if let Some(q) = get_quadrant(sy, sx, dy, dx, npy, npx) {
            quadrants[q] += 1;
        }
    }

    out.println(quadrants.iter().product::<i32>());
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
