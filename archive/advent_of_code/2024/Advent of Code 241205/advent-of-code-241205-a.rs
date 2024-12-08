//{"name":"advent-of-code-241205-a","group":"Advent of Code 241205","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"advent-of-code-241205-a"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::collections::HashMap;
use std::collections::HashSet;

type PreCalc = ();

fn solve(input: &mut Input, output: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut number_requirements: HashMap<i32, HashSet<i32>> = HashMap::new();
    let mut first_phase = true;
    let mut ans = 0;

    'read: while !input.is_empty() {
        let line = input.read_line();

        if first_phase && line.contains("|") {
            let (n1, n2) = line
                .split_once('|')
                .map(|(a, b)| (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()))
                .unwrap();

            let requirement = number_requirements.entry(n2).or_default();
            requirement.insert(n1);
            number_requirements.entry(n1).or_default();
        } else if first_phase {
            // will only run once right after the input shifts to question mode
            first_phase = false;
        }

        if !first_phase {
            let nums = line
                .split(",")
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            let seen: HashSet<i32> = HashSet::from([nums[0]]);
            let mut debt: HashSet<i32> = number_requirements.get(&nums[0]).unwrap().clone();

            for num in &nums[1..] {
                if debt.contains(num) {
                    continue 'read;
                }
                for num_req in &number_requirements[num] {
                    if !seen.contains(num_req) {
                        debt.insert(*num_req);
                    }
                }
            }

            // nums is safe; get the middle number & add to ans
            ans += nums[nums.len() / 2];
        }
    }

    output.print_line(ans);
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
