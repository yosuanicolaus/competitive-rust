//{"name":"D - Cubes","group":"AtCoder - OMRON Corporation Programming Contest 2025 (AtCoder Beginner Contest 397)","url":"https://atcoder.jp/contests/abc397/tasks/abc397_d","interactive":false,"timeLimit":2000,"tests":[{"input":"397\n","output":"12 11\n"},{"input":"1\n","output":"-1\n"},{"input":"39977273855577088\n","output":"342756 66212\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DCubes"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::collections::HashMap;

type PreCalc = ();

// WA on 40% of the test cases, the rest if AC.
// What if we need much bigger number than 10^18 to get the big numbers close to 10^18?
// This solution is already time-heavy (around, 1.1s, limit is 2s)
// Probably, we can't use the HashMap, and should use binary search on the nums array.

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let x = input.read_i128();
    let mut nums: Vec<i128> = vec![];
    let mut seen_idx: HashMap<i128, i128> = HashMap::new();
    let mut i = 1i128;
    let mut a = 1i128;
    let mut found = false;

    while a <= 10i128.pow(18) {
        nums.push(a);
        seen_idx.insert(a, i);
        i += 1;
        a = i.pow(3);
    }

    for i in 0..nums.len() {
        let a = nums[i];
        let mut b = x - a;
        if b <= 0 {
            b = a - x;
        }
        if seen_idx.contains_key(&b) {
            found = true;
            if a > b {
                out.println((i + 1, seen_idx[&b]));
            } else {
                out.println((seen_idx[&b], i + 1));
            }
            break;
        }
    }

    if !found {
        out.println(-1);
    }
    dbg!(nums.len());
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
    tester::run_main();
}
//END MAIN
