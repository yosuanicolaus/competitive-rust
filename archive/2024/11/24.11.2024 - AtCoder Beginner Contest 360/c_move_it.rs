//{"name":"C - Move It","group":"AtCoder - AtCoder Beginner Contest 360","url":"https://atcoder.jp/contests/abc360/tasks/abc360_c","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n2 2 3 3 5\n33 40 2 12 16\n","output":"35\n"},{"input":"12\n3 6 7 4 12 4 8 11 11 1 8 11\n3925 9785 9752 3587 4013 1117 3937 7045 6437 6208 3391 6309\n","output":"17254\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CMoveIt"}}}

use std::cmp::max;
use std::cmp::min;
use std::collections::HashMap;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_int_vec(n);
    let w = input.read_int_vec(n);

    let mut box_weights: HashMap<i32, i32> = HashMap::new();
    let mut ans = 0;

    for i in 0..n {
        if box_weights.contains_key(&a[i]) {
            // compare; drop the lighter one & save the heavier one
            ans += min(box_weights[&a[i]], w[i]);
            box_weights.insert(a[i], max(box_weights[&a[i]], w[i]));
        } else {
            box_weights.insert(a[i], w[i]);
        }
    }

    out.print(ans);
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
