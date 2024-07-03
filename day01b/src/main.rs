use std::fs;

pub fn main() {
    //     // read input
//     let input = r#"two1nine
// eightwothree
// abcone2threexyz
// xtwone3four
// 4nineeightseven2
// zoneight234
// 7pqrstsixteen
// "#;

    // read input file
    let input = fs::read_to_string("../inputs/day01.txt").unwrap();

    // parse input
    // let lines: Vec<_> = input.split('\n').collect();

    // process input
    // println!("{:?}", lines);

    let sum: u32 = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.to_string()
                .replace("one", "one1one")
                .replace("two", "two2two")
                .replace("three", "three3three")
                .replace("four", "four4four")
                .replace("five", "five5five")
                .replace("six", "six6six")
                .replace("seven", "seven7seven")
                .replace("eight", "eight8eight")
                .replace("nine", "nine9nine")
        })
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .map(|vec| 10 * vec.first().unwrap() + vec.last().unwrap())
        .sum();

    println!("{}", sum);
}
