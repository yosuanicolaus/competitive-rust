//{"name":"C - Variety Split Easy","group":"AtCoder - OMRON Corporation Programming Contest 2025 (AtCoder Beginner Contest 397)","url":"https://atcoder.jp/contests/abc397/tasks/abc397_c","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n3 1 4 1 5\n","output":"5\n"},{"input":"10\n2 5 6 5 2 1 7 9 7 2\n","output":"8\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CVarietySplitEasy"}}}

use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let nums = input.read_vec_int(n);

    let mut right_counts: HashMap<i32, i32> = HashMap::new();
    let mut left_seen: HashSet<i32> = HashSet::new();

    let mut ans = 0;
    let mut best_ans = 0;

    for num in &nums {
        *right_counts.entry(*num).or_insert(0) += 1;
        if right_counts[num] == 1 {
            ans += 1;
        }
    }

    for num in nums {
        if left_seen.contains(&num) {
            //
        } else {
            left_seen.insert(num);
            ans += 1;
        }

        // right_counts[&num] -= 1;
        *right_counts.entry(num).or_default() -= 1;
        if right_counts[&num] == 0 {
            ans -= 1;
        }

        best_ans = cmp::max(ans, best_ans);
    }

    out.println(best_ans);
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
