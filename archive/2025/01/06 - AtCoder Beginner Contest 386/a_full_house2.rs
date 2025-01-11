//{"name":"A - Full House 2","group":"AtCoder - AtCoder Beginner Contest 386","url":"https://atcoder.jp/contests/abc386/tasks/abc386_a","interactive":false,"timeLimit":2000,"tests":[{"input":"7 7 7 1\n","output":"Yes\n"},{"input":"13 12 11 10\n","output":"No\n"},{"input":"3 3 5 5\n","output":"Yes\n"},{"input":"8 8 8 8\n","output":"No\n"},{"input":"1 3 4 1\n","output":"No\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AFullHouse2"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut a = input.read_int_vec(4);
    a.sort();

    if (a[0] != a[1] || a[0] != a[2] || a[0] != a[3]) // make sure all the cards are not the same
        && ((a[0] == a[1] && a[1] == a[2])
            || (a[1] == a[2] && a[2] == a[3])
            || (a[0] == a[1] && a[2] == a[3]))
    {
        out.println("Yes");
    } else {
        out.println("No");
    }
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
