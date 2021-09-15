use std::io::{self, BufRead, Lines, StdinLock};

#[derive(Copy, Clone, Debug)]
struct TestDataStruct {
    n: i32,
    k: i32,
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let tests: i32 = lines.next().unwrap().unwrap().parse().unwrap();
    let mut i = 1i32;
    while i <= tests {
        let test_data = parse_test_case(&mut lines);
        println!("Parsed Data: {:?}", test_data);
        solve_problem(test_data);
        i += 1;
    }
}

fn parse_test_case(lines: &mut Lines<StdinLock>) -> TestDataStruct {
    //Parse line by line here, get the line's string with:
    //lines.next().unwrap().unwrap()

    //Split up by spaces and parse into numbers:
    // let line_1_nums: Vec<i32> = lines
    // .next()
    // .unwrap()
    // .unwrap()
    // .split_whitespace()
    // .map(|el: &str| el.parse::<i32>().unwrap())
    // .collect();

    let line_1_nums: Vec<i32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|el: &str| el.parse::<i32>().unwrap())
        .collect();
    let mut test_data = TestDataStruct { n: 0, k: 0 };
    test_data.n = line_1_nums[0];
    test_data.k = line_1_nums[1];

    println!("Line 1, n: {:?}, k: {:?}", test_data.n, test_data.k);
    println!("Line 2: {:?}", lines.next().unwrap().unwrap());
    println!("Done parsing test case");

    test_data
}

fn solve_problem(mut test_data: TestDataStruct) {
    test_data.n += 1;
    println!("Solved test!: {:?}", test_data);
}
