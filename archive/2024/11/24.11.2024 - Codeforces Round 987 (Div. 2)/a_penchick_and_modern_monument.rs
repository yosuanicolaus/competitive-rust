//{"name":"A. Penchick and Modern Monument","group":"Codeforces - Codeforces Round 987 (Div. 2)","url":"https://codeforces.com/problemset/problem/2031/A","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n5\n5 4 3 2 1\n3\n2 2 1\n1\n1\n","output":"4\n1\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"APenchickAndModernMonument"}}}

use std::cmp;
use std::collections::HashMap;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let h = input.read_int_vec(n);

    let mut counts: HashMap<i32, i32> = HashMap::new();
    let mut best_count = 0;

    for num in h {
        let count = counts.entry(num).or_insert(0);
        *count += 1;
        best_count = cmp::max(best_count, *count);
    }

    out.print_line(n as i32 - best_count);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
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
        TaskType::Classic => {
            input.is_empty()
        }
        TaskType::Interactive => true,
    }
}


//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
