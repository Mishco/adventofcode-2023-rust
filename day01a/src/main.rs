use regex::Regex;
use std::fs;
fn main() {
//     // read input
//     let input = r#"1abc2
// pqr3stu8vwx
// a1b2c3d4e5f
// treb7uchet"#;

    // read input file
    let input = fs::read_to_string("../inputs/day01.txt").unwrap();

    // parse input
    let lines: Vec<_> = input.split('\n').collect();

    // process input
    // println!("{:?}", lines);

    let re = Regex::new(r"\d").unwrap();
    let mut sum: usize = 0;
    for line in lines {
        let mut digits = re
            .find_iter(line)
            .map(|mat| mat.as_str().chars().next().unwrap());

        let first_digit = digits.next().unwrap_or('0');
        let last_digit = digits.last().unwrap_or(first_digit);

        let act_val = format!("{}{}", first_digit, last_digit);
        sum += act_val.parse::<usize>().unwrap_or(0);
    }

    // print result
    println!("{}", sum);
}
