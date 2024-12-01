use std::fs;
use std::str::FromStr;

fn main() {
    let file_path = "input.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let (mut a, mut b): (Vec<_>, Vec<_>) = contents
        .lines()
        .map(|line| {
            let ss: Vec<u32> = line.split_whitespace()
            .map(u32::from_str)
            .map(Result::unwrap)
            .collect();
            (ss[0], ss[1])
        })
        .unzip();

    a.sort();
    b.sort();

    let ans1: u32 = a.iter().zip(b.iter()).map(|x| x.0.abs_diff(*x.1)).sum();
    println!("{}", ans1);

    let ans2: u32 = a
        .iter()
        .map(|&x| x * b.iter().filter(|&y| *y == x).count() as u32)
        .sum();
    println!("{}", ans2);
}
