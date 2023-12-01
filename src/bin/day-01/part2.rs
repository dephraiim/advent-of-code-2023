// given up on day 1

use std::fs;
use std::io;

pub fn run() -> io::Result<i32> {
    let input = fs::read_to_string("src/bin/day-01/input-2.txt")?;
    let lines: Vec<String> = input.split("\n").map(|s: &str| s.to_string()).collect();

    let mut total_sum = 0;

    for line in lines {
        let numbers = vec![
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];

        // mutable vector created to store the occurances
        let mut occurrences: Vec<(usize, &str)> = Vec::new();

        for number in &numbers {
            let mut start = 0;
            while let Some(pos) = line[start..].find(number) {
                occurrences.push((start + pos, number));
                start += pos + 1;
            }
        }

        occurrences.sort_by_key(|&(index, _)| index);

        let numbers_present: Vec<&str> = occurrences
            .iter()
            .map(|&(_, number)| match number {
                "one" => "1",
                "two" => "2",
                "three" => "3",
                "four" => "4",
                "five" => "5",
                "six" => "6",
                "seven" => "7",
                "eight" => "8",
                "nine" => "9",
                _ => "0",
            })
            .collect();

        println!("{:?}", numbers_present);

        // let numbers = vec![
        //     "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        // ];

        // let line_copy: String = line.to_owned();

        // let mut numbers_present: Vec<&str> = Vec::new();

        // for number in numbers {
        //     if line_copy.contains(number) {
        //         numbers_present.push(match number {
        //             "one" => "1",
        //             "two" => "2",
        //             "three" => "3",
        //             "four" => "4",
        //             "five" => "5",
        //             "six" => "6",
        //             "seven" => "7",
        //             "eight" => "8",
        //             "nine" => "9",
        //             _ => "0",
        //         });
        //     }
        // }

        // println!("{:?}", numbers_present);

        // println!("{}", line_copy);
        // println!();

        // let letters: Vec<char> = line.chars().filter(|c| c.is_numeric()).collect();

        // let first_number = letters.first().unwrap_or(&'0').to_owned();
        // let last_number = letters.last().unwrap_or(&'0').to_owned();

        // let mut number_string = String::new();

        // number_string.push(first_number);
        // number_string.push(last_number);

        // println!("letters: {:?} | number_string {:?}", letters, number_string);

        // let line_number_parsed: i32 = number_string.parse().unwrap();

        // total_sum = total_sum + line_number_parsed;
    }

    Ok(total_sum)
}
