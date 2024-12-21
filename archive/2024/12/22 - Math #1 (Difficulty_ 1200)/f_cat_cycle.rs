//{"name":"F. Cat Cycle","group":"Codeforces - Math #1 (Difficulty: 1200)","url":"https://codeforces.com/group/JESCgZZ8qn/contest/333990/problem/F","interactive":false,"timeLimit":1000,"tests":[{"input":"7\n2 1\n2 2\n3 1\n3 2\n3 3\n5 5\n69 1337\n","output":"1\n2\n1\n3\n2\n2\n65\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FCatCycle"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_long();
    let k = input.read_long();

    if n % 2 == 0 {
        out.println((k - 1) % n + 1);
    } else {
        let t = n / 2;
        let i = (k - 1) / t;
        let ans = (k + i - 1) % n + 1;
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
