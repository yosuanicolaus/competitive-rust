//{"name":"A. Same Differences","group":"Codeforces - Math #1 (Difficulty: 1200)","url":"https://codeforces.com/group/JESCgZZ8qn/contest/333990/problem/A","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n6\n3 5 1 4 6 6\n3\n1 2 3\n4\n1 3 3 4\n6\n1 6 3 4 5 6\n","output":"1\n3\n3\n10\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ASameDifferences"}}}

use std::collections::HashMap;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_int_vec(n);
    let mut start_count: HashMap<i32, u32> = HashMap::new();

    for (i, &num) in a.iter().enumerate() {
        let zero_start = i as i32 - num;
        let c = start_count.entry(zero_start).or_insert(0);
        *c += 1;
    }

    let mut ans: u64 = 0;
    for &c in start_count.values() {
        if c > 1 {
            let nc = c as u64;
            ans += nc * (nc - 1) / 2;
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
