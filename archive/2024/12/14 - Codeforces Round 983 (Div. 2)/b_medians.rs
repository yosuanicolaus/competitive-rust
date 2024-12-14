//{"name":"B. Medians","group":"Codeforces - Codeforces Round 983 (Div. 2)","url":"https://codeforces.com/problemset/problem/2032/B","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n1 1\n3 2\n3 3\n15 8\n","output":"1\n1\n3\n1 2 3\n-1\n5\n1 4 7 10 13\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BMedians"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_int();
    let k = input.read_int();

    if n == 1 {
        out.println("1\n1");
    } else if k == 1 || k == n {
        out.println(-1);
    } else {
        out.println(3);
        if k % 2 == 0 {
            out.println(vec![1, k, k + 1]);
        } else {
            out.println(vec![1, k - 1, k + 2]);
        }
    }
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
