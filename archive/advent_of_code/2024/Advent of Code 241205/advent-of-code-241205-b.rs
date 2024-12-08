//{"name":"advent-of-code-241205-b","group":"Advent of Code 241205","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"advent-of-code-241205-b"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::test_type::TaskType;
use algo_lib::misc::test_type::TestType;
use std::collections::HashMap;
use std::collections::HashSet;

type PreCalc = ();

fn get_ordered_nums_mid(bad_nums: &Vec<i32>, number_requirements: &HashMap<i32, HashSet<i32>>) -> i32 {
    let mut good_nums: Vec<i32> = vec![];
    let mut nums_to_add: HashSet<i32> = bad_nums.iter().cloned().collect();

    while !nums_to_add.is_empty() {
        let mut num_to_remove: Option<i32> = None;
        'next_num: for test_num in &nums_to_add {
            for req_num in number_requirements[test_num].clone() {
                if nums_to_add.contains(&req_num) {
                    continue 'next_num;
                }
            }
            num_to_remove = Some(*test_num);
        }
        if let Some(num) = num_to_remove {
            nums_to_add.remove(&num);
            good_nums.push(num);
        } else {
            panic!("no num to remove?!");
        }
    }
    good_nums[good_nums.len() / 2]
}

fn is_safely_ordered(nums: &Vec<i32>, number_requirements: &HashMap<i32, HashSet<i32>>) -> bool {
    let seen: HashSet<i32> = HashSet::from([nums[0]]);
    let mut debt: HashSet<i32> = number_requirements.get(&nums[0]).unwrap().clone();

    for num in &nums[1..] {
        if debt.contains(num) {
            return false;
        }
        for num_req in &number_requirements[num] {
            if !seen.contains(num_req) {
                debt.insert(*num_req);
            }
        }
    }
    true
}

fn solve(input: &mut Input, output: &mut Output, _test_case: usize, _data: &mut PreCalc) {
    let mut number_requirements: HashMap<i32, HashSet<i32>> = HashMap::new();
    let mut first_phase = true;
    let mut ans = 0;

    while !input.is_empty() {
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

            if !is_safely_ordered(&nums, &number_requirements) {
                ans += get_ordered_nums_mid(&nums, &number_requirements);
            }
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
