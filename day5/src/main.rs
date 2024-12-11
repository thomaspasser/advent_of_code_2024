use std::fs;
use std::str::FromStr;

fn main() {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    // Part 1
    // Find first empty line
    let n = contents.lines().position(|l| l.is_empty()).unwrap();

    let ord_rules = contents.lines().take(n).map(|s| {
        s.split("|")
            .map(u32::from_str)
            .map(Result::unwrap)
            .collect::<Vec<_>>()
    });

    let updates = contents.lines().skip(n + 1);

    let mut ans1 = 0;
    let mut ans2 = 0;
    for v in updates {
        let mut valid = true;
        let vec: Vec<u32> = v
            .split(",")
            .map(u32::from_str)
            .map(Result::unwrap)
            .collect();

        for rule in ord_rules.clone() {

            // Find index of right, left val
            let i1 = vec.iter().position(|&v| v == rule[0]);
            let i2 = vec.iter().position(|&v| v == rule[1]);
            if i1.is_none() || i2.is_none() {
                continue;
            }
            // Check
            if i1.unwrap() > i2.unwrap() {
                valid = false;
                break;
            }
        }
        if valid {
            ans1 += vec[vec.len() / 2];
        } else {
            // For part two
            let mut vec2 =  vec![0u32; vec.len()];

            for p in vec.iter() {
                let n = ord_rules.clone()
                    .filter(|v| v[1] == *p)
                    .filter(|v| vec.contains(&v[0]))
                    .count();
                vec2[n] = *p;
            }
            ans2 += vec2[vec2.len() / 2];
        }
    }

    println!("{}", ans1);
    println!("{}", ans2);
}
