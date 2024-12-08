//{"name":"advent-of-code-241202-a","group":"Advent of Code 241202","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"advent-of-code-241202-a"}}}

use std::collections::HashSet;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn get_adjacent_diff(report: &Vec<i32>, upper_idx: usize) -> i32 {
    report[upper_idx] - report[upper_idx - 1]
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut safe_reports = 0;
    let increasing_set = HashSet::from([1, 2, 3]);
    let decreasing_set = HashSet::from([-1, -2, -3]);

    'read_line: while !input.is_exhausted() {
        let input_line = input.read_line();
        let report: Vec<i32> = input_line
            .split(' ')
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        let set_to_use: &HashSet<i32>;

        if increasing_set.contains(&get_adjacent_diff(&report, 1)) {
            set_to_use = &increasing_set;
        } else if decreasing_set.contains(&get_adjacent_diff(&report, 1)) {
            set_to_use = &decreasing_set;
        } else {
            continue;
        }

        for i in 2..report.len() {
            if !set_to_use.contains(&get_adjacent_diff(&report, i)) {
                continue 'read_line;
            }
        }
        safe_reports += 1;
    }

    out.print(safe_reports);
}

pub static TEST_TYPE: TestType = TestType::Single;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut pre_calc = ();

    match TEST_TYPE {
        TestType::Single => solve(&mut input, &mut output, 1, &mut pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 1..=t {
                solve(&mut input, &mut output, i, &mut pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &mut pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    match TASK_TYPE {
        TaskType::Classic => input.is_empty(),
        TaskType::Interactive => true,
    }
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
