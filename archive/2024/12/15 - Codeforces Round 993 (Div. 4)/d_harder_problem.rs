//{"name":"D. Harder Problem","group":"Codeforces - Codeforces Round 993 (Div. 4)","url":"https://codeforces.com/contest/2044/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n2\n1 2\n4\n1 1 1 2\n8\n4 5 5 5 1 1 2 1\n10\n1 1 2 2 1 1 3 3 1 1\n","output":"1 2\n1 1 2 2\n4 5 5 1 1 2 2 3\n1 8 2 2 1 3 3 9 1 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DHarderProblem"}}}

use std::collections::HashSet;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_int_vec(n);
    let mut seen: HashSet<i32> = HashSet::new();
    let mut usable: HashSet<i32> = (1..=(n as i32)).into_iter().collect();
    let mut ans: Vec<i32> = vec![];

    for num in &a {
        usable.remove(num);
    }

    let usable: Vec<i32> = usable.iter().map(|a| a.clone()).collect();
    let mut ui: usize = 0;

    for &num in &a {
        if seen.contains(&num) {
            ans.push(usable[ui]);
            ui += 1;
        } else {
            ans.push(num);
            seen.insert(num);
        }
    }

    out.println(ans);
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
