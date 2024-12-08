//{"name":"advent-of-code-241202-b","group":"Advent of Code 241202","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"advent-of-code-241202-b"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn report_is_safe(report: &Vec<i32>, skip_idx: Option<usize>, used_skip: bool) -> bool {
    if used_skip {
        return false;
    }

    let mut first_compare = true;
    let mut prev_diff = 0;
    let have_skip = skip_idx.is_some();

    for lo_idx in 0..report.len() - 1 {
        let mut hi_idx = lo_idx + 1;

        // readjust lo/hi idx if skip_idx is within them
        if let Some(skip_idx) = skip_idx {
            if skip_idx == lo_idx {
                continue;
            } else if skip_idx == hi_idx {
                // compare lo_idx and hi_idx + 1 if exist
                if hi_idx + 1 < report.len() {
                    hi_idx += 1;
                } else {
                    // can't compare, we're at the end; it's finished!
                    return true;
                }
            }
        }

        let diff = report[hi_idx] - report[lo_idx];
        if diff == 0 {
            // either skipping the lo / hi doesn't matter
            return report_is_safe(report, Some(lo_idx), have_skip);
        } else if diff > 3 || diff < -3 {
            // no matter what, one of the 2 idx must be skipped
            return report_is_safe(report, Some(lo_idx), have_skip)
                || report_is_safe(report, Some(hi_idx), have_skip);
        }

        // safe diff (1 ~ 3 or -3 ~ -1), but still need to compare if not first
        if first_compare {
            first_compare = false;
        } else {
            // compare with previous diff
            if (diff > 0 && prev_diff < 0) || (diff < 0 && prev_diff > 0) {
                // both are "safe", but with different direction
                if lo_idx > 0 && report_is_safe(report, Some(lo_idx - 1), have_skip) {
                    return true;
                }
                return report_is_safe(report, Some(lo_idx), have_skip)
                    || report_is_safe(report, Some(hi_idx), have_skip);
            }
        }
        prev_diff = diff;
    }

    true
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut safe_reports = 0;

    while !input.is_empty() {
        let input_line = input.read_line();
        let report: Vec<i32> = input_line
            .split(' ')
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        if report_is_safe(&report, None, false) {
            safe_reports += 1;
        }
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
