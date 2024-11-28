//{"name":"A. Fair Playoff","group":"Codeforces - Educational Codeforces Round 110 (Rated for Div. 2)","url":"https://codeforces.com/problemset/problem/1535/A","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n3 7 9 5\n4 5 6 9\n5 3 8 1\n6 5 3 2\n","output":"YES\nNO\nYES\nNO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AFairPlayoff"}}}

use std::cmp::max;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut skills = input.read_int_vec(4);
    let winner1 = max(skills[0], skills[1]);
    let winner2 = max(skills[2], skills[3]);
    let mut winners = [winner1, winner2];
    winners.sort();
    skills.sort();

    out.print_line(if winners == skills[2..] {"YES"} else {"NO"});
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
