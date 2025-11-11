//{"name":"D. Yet Another Array Problem","group":"Codeforces - Codeforces Round 1062 (Div. 4)","url":"https://codeforces.com/problemset/problem/2167/D","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n1\n1\n4\n6 6 12 12\n3\n24 120 210\n4\n2 4 6 10\n","output":"2\n5\n5\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DYetAnotherArrayProblem"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = Vec<i64>;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, primes: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_vec_long(n);

    for prime_num in primes {
        for a_num in &a {
            if a_num % *prime_num != 0 {
                out.println(*prime_num);
                return;
            }
        }
    }
    out.println(-1);
}

pub static TEST_TYPE: TestType = TestType::MultiNumber;
pub static TASK_TYPE: TaskType = TaskType::Classic;

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let mut prime_idxs: [bool; 54] = [true; 54];
    let mut primes: Vec<i64> = Vec::new();

    for i in 2..=53 {
        if prime_idxs[i] {
            primes.push(i as i64);
            for j in (i+i..=53).step_by(i) {
                prime_idxs[j] = false;
            }
        }
    }

    let mut pre_calc = primes;

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
