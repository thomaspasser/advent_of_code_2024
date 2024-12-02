use std::fs;
use std::str::FromStr;

fn diff(v: &Vec<i32>) -> Vec<i32> {
    v.windows(2).map(|s: &[i32]| s[1] - s[0]).collect()
}

fn is_safe(v: &Vec<i32>) -> bool {
    let d = diff(v);
    // all increasing
    let test1 = d.iter().all(|&x| x > 0);
    // or all decreasing.
    let test2 = d.iter().all(|&x| x < 0);

    // Any two adjacent levels differ by at least one and at most three.
    let test3 = d
        .clone()
        .into_iter()
        .map(i32::abs)
        .all(|x| x >= 1 && x <= 3);

    (test1 || test2) && test3
}

fn tolerant_is_safe(v: &Vec<i32>) -> bool {
    // Pretty ugly
    for i in 0..v.len() {
        let mut temp = v.clone();
        temp.remove(i);
        if is_safe(&temp) {
            return true;
        }
    }
    false
}

fn main() {
    let file_path = "input.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let temp = contents.lines().map(|line| {
        line.split_whitespace()
            .map(i32::from_str)
            .map(Result::unwrap)
            .collect::<Vec<_>>()
    });

    let ans1 = temp.clone().filter(is_safe).count();
    println!("{}", ans1);

    let ans2 = temp.filter(tolerant_is_safe).count();
    println!("{}", ans2);
}
