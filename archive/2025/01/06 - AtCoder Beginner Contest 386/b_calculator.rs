//{"name":"B - Calculator","group":"AtCoder - AtCoder Beginner Contest 386","url":"https://atcoder.jp/contests/abc386/tasks/abc386_b","interactive":false,"timeLimit":2000,"tests":[{"input":"1000000007\n","output":"6\n"},{"input":"998244353\n","output":"9\n"},{"input":"32000\n","output":"4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BCalculator"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let s = input.read_line().chars().collect::<Vec<char>>();
    let mut i = 0usize;
    let mut ans = 0;

    while i < s.len() {
        if i + 1 < s.len() && s[i] == '0' && s[i + 1] == '0' {
            i += 2;
        } else {
            i += 1;
        }
        ans += 1;
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
