use std::fs;
use std::io;

pub fn run() -> io::Result<i32> {
    let input = fs::read_to_string("src/bin/day-01/input-1.txt")?;
    let lines: Vec<String> = input.split("\n").map(|s: &str| s.to_string()).collect();

    let mut total_sum = 0;

    for line in lines {
        let letters: Vec<char> = line.chars().filter(|c| c.is_numeric()).collect();

        let first_number = letters.first().unwrap().to_owned();
        let last_number = letters.last().unwrap().to_owned();

        let mut number_string = String::new();

        number_string.push(first_number);
        number_string.push(last_number);

        // println!("letters: {:?} | number_string {:?}", letters, number_string);

        let line_number_parsed: i32 = number_string.parse().unwrap();

        total_sum = total_sum + line_number_parsed;
    }

    Ok(total_sum)
}
