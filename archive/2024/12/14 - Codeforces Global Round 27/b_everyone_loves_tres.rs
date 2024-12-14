//{"name":"B. Everyone Loves Tres","group":"Codeforces - Codeforces Global Round 27","url":"https://codeforces.com/problemset/problem/2035/B","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n1\n2\n3\n4\n5\n7\n","output":"-1\n66\n-1\n3366\n36366\n3336366\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BEveryoneLovesTres"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_int();

    if n == 1 || n == 3 {
        out.println(-1);
    } else if n == 2 {
        out.println(66);
    } else if n % 2 == 0 {
        let ans = "3".repeat(n as usize - 4) + "3366";
        out.println(ans);
    } else {
        let ans = "3".repeat(n as usize - 4) + "6366";
        out.println(ans);
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
