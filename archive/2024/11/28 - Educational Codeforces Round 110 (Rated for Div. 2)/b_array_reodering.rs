//{"name":"B. Array Reodering","group":"Codeforces - Educational Codeforces Round 110 (Rated for Div. 2)","url":"https://codeforces.com/problemset/problem/1535/B","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n4\n3 6 5 3\n2\n1 7\n5\n1 4 2 4 1\n","output":"4\n0\n9\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BArrayReodering"}}}

use std::collections::HashMap;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;

use algo_lib::misc::test_type::TestType;

type PreCalc = ();

// TODO: wrong answer... why? let's do this again later

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_size();
    let a = input.read_int_vec(n);
    let mut total_even: i32 = 0;
    let mut odd_counts: HashMap<i32, i32> = HashMap::new();

    for num in a {
        if num % 2 == 0 {
            total_even += 1;
        } else {
            let oc = odd_counts.entry(num).or_insert(0);
            *oc += 1;
        }
    }

    let n = n as i32;
    let init_ans = total_even * ((n - total_even) + (n - 1)) / 2;
    let mut odd_ans = 0;

    for (odd_num, odd_c) in odd_counts {
        if odd_num > 1 && odd_c > 1 {
            odd_ans += (odd_c - 1) * (1 + (odd_c - 1)) / 2;
        }
    }
    out.print_line(init_ans + odd_ans);
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
