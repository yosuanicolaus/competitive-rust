//{"name":"B. Shohag Loves Strings","group":"Codeforces - CodeTON Round 9 (Div. 1 + Div. 2, Rated, Prizes!)","url":"https://codeforces.com/problemset/problem/2039/B","interactive":false,"timeLimit":1000,"tests":[{"input":"5\ndcabaac\na\nyouknowwho\ncodeforces\nbangladesh\n","output":"abaa\n-1\nyouknowwho\neforce\nbang\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BShohagLovesStrings"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let s = input.read_line().chars().collect::<Vec<char>>();

    if s.len() == 1 {
        out.println(-1);
    } else {
        for i in 0..s.len() - 1 {
            if s[i] == s[i + 1] {
                out.println(&s[i..i + 2].iter().collect::<String>());
                return;
            }
        }
        for i in 0..s.len() - 2 {
            if s[i] != s[i + 1] && s[i] != s[i + 2] {
                out.println(&s[i..i + 3].iter().collect::<String>());
                return;
            }
        }
        out.println(-1);
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
