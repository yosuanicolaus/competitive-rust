//{"name":"B - Ticket Gate Log","group":"AtCoder - OMRON Corporation Programming Contest 2025 (AtCoder Beginner Contest 397)","url":"https://atcoder.jp/contests/abc397/tasks/abc397_b","interactive":false,"timeLimit":2000,"tests":[{"input":"ioi\n","output":"1\n"},{"input":"iioo\n","output":"2\n"},{"input":"io\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BTicketGateLog"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let s = input.read_vec_char();
    let mut ans = 0;

    if s[0] == 'o' {
        ans += 1;
    }
    if s[s.len() - 1] == 'i' {
        ans += 1;
    }

    for i in 0..s.len() - 1 {
        let j = i + 1;
        if s[i] == s[j] {
            ans += 1;
        }
    }

    out.println(ans);
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
