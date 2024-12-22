//{"name":"C. Preparing for the Exam","group":"Codeforces - Codeforces Round 995 (Div. 3)","url":"https://codeforces.com/contest/2051/problem/C","interactive":false,"timeLimit":1500,"tests":[{"input":"4\n4 4 3\n1 2 3 4\n1 3 4\n5 4 3\n1 2 3 4\n1 3 4\n4 4 4\n1 2 3 4\n1 2 3 4\n2 2 1\n1 2\n2\n","output":"0100\n0000\n1111\n10\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CPreparingForTheExam"}}}

use std::collections::HashSet;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n: i32 = input.read();
    let m: usize = input.read();
    let k: usize = input.read();
    let mut qabsent = vec![];
    let mut answerable: HashSet<i32> = (1..=n).into_iter().collect();

    for _ in 0..m {
        qabsent.push(input.read_int());
    }
    for _ in 0..k {
        answerable.remove(&input.read_int());
    }

    let mut ans: Vec<char>;
    if k as i32 == n {
        ans = vec!['1'; m];
    } else if k as i32 + 2 <= n {
        ans = vec!['0'; m];
    } else {
        ans = vec![];
        for q in qabsent {
            if answerable.contains(&q) {
                ans.push('1');
            } else {
                ans.push('0');
            }
        }
    }

    out.println(ans.iter().collect::<String>());
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
