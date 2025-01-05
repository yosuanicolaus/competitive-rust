//{"name":"C. Bewitching Stargazer","group":"Codeforces - Good Bye 2024: 2025 is NEAR","url":"https://codeforces.com/contest/2053/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n7 2\n11 3\n55 13\n5801 6\n8919 64\n8765432 1\n","output":"12\n18\n196\n1975581\n958900\n38416403456028\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CBewitchingStargazer"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;

type PreCalc = ();

// not solved yet ._.

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let n = input.read_i128();
    let k = input.read_i128();
    let mut ans: i128 = 0;

    let mut z = n / 2 + 1;
    let mut found = n % 2 == 1;
    let mut cn = n;
    let mut i = 1;

    while cn >= k {
        if cn % 2 == 1 {
            if !found {
                found = true;
                z = cn / 2;
                i = 1;
                ans += z;
            } else {
                ans += z * i;
            }
        }

        cn /= 2;
        i *= 2;
        dbg!(ans, cn, i);
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
    tester::run_main();
}
//END MAIN
