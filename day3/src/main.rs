use std::fs;

use regex::Regex;

fn main() {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    // Part 1
    let ans1: i32 = re
        .captures_iter(&contents)
        .map(|c| {
            let (_, [a, b]) = c.extract();
            a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap()
        })
        .sum();

    println!("{}", ans1);

    // Part 2
    let mut ans2: i32 = 0;
    let mut start: usize = 0;
    let mut end: usize;
    loop {
        // Find next don't()

        match contents[start..].find("don't()") {
            Some(res) => end = res + start,
            None => end = contents.len(),
        }

        ans2 += re
            .captures_iter(&contents[start..end])
            .map(|c| {
                let (_, [a, b]) = c.extract();
                a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap()
            })
            .sum::<i32>();

        // Look for next do()
        match contents[end..].find("do()") {
            Some(res) => start = res + end,
            None => break,
        }
    }

    println!("{}", ans2);
}
