use std::fs;

fn count_in_line(s: &str, substr: &str) -> u32 {
    s.as_bytes()
        .windows(substr.len())
        .filter(|&w| w == substr.as_bytes())
        .count()
        .try_into()
        .unwrap()
}

fn line_as_char_array(s: &str) -> Result<[char; 140], String> {
    let letters: Vec<char> = s.chars().collect();
    match letters.try_into() {
        Ok(result) => Ok(result),
        Err(_e) => Err("Not possible".to_string()),
    }
}

fn main() {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    // Part 1
    let mut ans1: u32 = 0;
    ans1 += contents
        .lines()
        .map(|s| count_in_line(s, "XMAS"))
        .sum::<u32>();
    ans1 += contents
        .lines()
        .map(|s| count_in_line(s, "SAMX"))
        .sum::<u32>();

    let mut temp: Vec<[char; 140]> = contents
        .lines()
        .map(|s| line_as_char_array(s).unwrap())
        .collect();

    let A = temp.as_mut_slice();

    // vertical
    for i in 0..140 {
        let s = A.into_iter().map(|l| l[i]).collect::<String>();
        ans1 += count_in_line(&s, "XMAS");
        ans1 += count_in_line(&s, "SAMX");

        // Diagonal \, above diagonal
        let s = A
            .into_iter()
            .enumerate()
            .filter(|(j, _l)| i + j < 140)
            .map(|(j, l)| l[i + j])
            .collect::<String>();

        ans1 += count_in_line(&s, "XMAS");
        ans1 += count_in_line(&s, "SAMX");

        // Diagonal \, below diagonal
        let s = A
            .into_iter()
            .skip(1)
            .enumerate()
            .filter(|(j, _l)| (*j as i32) - (i as i32) >= 0)
            .map(|(j, l)| l[j - i])
            .collect::<String>();

        ans1 += count_in_line(&s, "XMAS");
        ans1 += count_in_line(&s, "SAMX");

        // Diagonal /, top half
        let s = A
            .into_iter()
            .enumerate()
            .filter(|(j, _l)| (i as i32) - (*j as i32) >= 0)
            .map(|(j, l)| l[i - j])
            .collect::<String>();
        ans1 += count_in_line(&s, "XMAS");
        ans1 += count_in_line(&s, "SAMX");

        // Diagonal /, bottom half
        let s = A
            .into_iter()
            .skip(1)
            .enumerate()
            .filter(|(j, _l)| 139 + (i as i32) - (*j as i32) < 140)
            .map(|(j, l)| l[139 + i - j])
            .collect::<String>();
        ans1 += count_in_line(&s, "XMAS");
        ans1 += count_in_line(&s, "SAMX");
    }
    println!("{}", ans1);

    // Part 2
    // Find all 'A's
    let mut ans2 = 0;
    for i in 1..139 {
        for j in 1..139 {
            if A[i][j] == 'A' {
                // println!("{}{}{}\r\n{}{}{}\r\n{}{}{}", A[i-1][j-1],  A[i-1][j],  A[i-1][j+1], A[i][j-1],  A[i][j],  A[i][j+1], A[i+1][j-1],  A[i+1][j],  A[i+1][j+1]);
                if (A[i - 1][j - 1] == 'M' && A[i + 1][j + 1] == 'S'
                    || A[i - 1][j - 1] == 'S' && A[i + 1][j + 1] == 'M')
                    && (A[i - 1][j + 1] == 'M' && A[i + 1][j - 1] == 'S'
                        || A[i - 1][j + 1] == 'S' && A[i + 1][j - 1] == 'M')
                {
                    ans2 += 1;
                }
            }
        }
    }
    println!("{}", ans2);
}
