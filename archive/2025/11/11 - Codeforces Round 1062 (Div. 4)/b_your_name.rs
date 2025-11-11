//{"name":"B. Your Name","group":"Codeforces - Codeforces Round 1062 (Div. 4)","url":"https://codeforces.com/problemset/problem/2167/B","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n7\nhumitsa mitsuha\n4\norhi hori\n6\naakima makima\n6\nnezuqo nezuko\n6\nmisaka mikasa\n","output":"YES\nYES\nNO\nNO\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BYourName"}}}

use std::collections::HashMap;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let mut name1: HashMap<char, usize> = HashMap::new();
    let mut name2: HashMap<char, usize> = HashMap::new();

    for _ in 0..n {
        let ch = input.read_char();
        *name1.entry(ch).or_insert(0) += 1;
    }

    for _ in 0..n {
        let ch = input.read_char();
        *name2.entry(ch).or_insert(0) += 1;
    }

    if name1 == name2 {
        out.println("YES");
    } else {
        out.println("NO");
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
        TaskType::Classic => {
            input.is_empty()
        }
        TaskType::Interactive => true,
    }
}

//START MAIN
mod tester;
fn main() {
    tester::run_main();
}
//END MAIN
