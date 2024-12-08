//{"name":"advent-of-code-241201-a","group":"Advent of Code 241201","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"advent-of-code-241201-a"}}}

use std::collections::HashMap;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut counts_1: HashMap<i32, i32> = HashMap::new();
    let mut counts_2: HashMap<i32, i32> = HashMap::new();

    while !input.is_exhausted() {
        let a = input.read_int();
        let b = input.read_int();

        let c1 = counts_1.entry(a).or_insert(0);
        let c2 = counts_2.entry(b).or_insert(0);
        *c1 += 1;
        *c2 += 1;
    }

    let mut ans = 0;

    for (c1k, c1v) in counts_1 {
        ans += c1v * c1k * counts_2.get(&c1k).unwrap_or(&0);
    }

    out.print(ans);
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
        TaskType::Classic => {
            input.is_empty()
        }
        TaskType::Interactive => true,
    }
}


//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
