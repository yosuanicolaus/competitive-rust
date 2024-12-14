use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

enum InputType {
    Array,   // e.g. `[1,3,4,5]`
    Grid,    // e.g. `[[1,3,2],[4,5,3]]`
    String,  // e.g. `"abcde"`
    Integer, // e.g. `123`
}

static INPUT_TYPE: InputType = InputType::Grid;

pub struct Solution {}

impl Solution {
    pub fn max_two_events(events: Vec<Vec<i32>>) -> i32 {
        0
    }
}

fn solve<T>(input: T, out: &mut Output) {
    let res = Solution::max_two_events(input);
    out.println(res);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    // TODO : handle selecting which InputType to use when building the file
    match INPUT_TYPE {
        InputType::Array => {
            let line = input.read_line().replace(" ", "");
            let vec_input: Vec<i32> = line[1..line.len() - 1]
                .split(',')
                .map(|x| x.parse::<i32>().unwrap())
                .collect();
            solve::<Vec<i32>>(vec_input, &mut output);
        }
        InputType::Grid => {
            let line = input.read_line().replace(" ", "");
            let grid_input: Vec<Vec<i32>> = line
                .split("],[")
                .map(|grid_line| {
                    grid_line
                        .replace("[", "")
                        .replace("]", "")
                        .split(",")
                        .map(|item| item.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>()
                })
                .collect();
            solve::<Vec<Vec<i32>>>(grid_input, &mut output);
        }
        InputType::String => {
            let str_input = input.read_line().replace('"', "");
            solve::<String>(str_input, &mut output);
        }
        InputType::Integer => {
            let int_input = input.read_int();
            solve::<i32>(int_input, &mut output);
        }
    }
    output.flush();
    input.is_empty()
}

//START MAIN
mod tester;
fn main() {
    tester::run_tests();
}
//END MAIN
