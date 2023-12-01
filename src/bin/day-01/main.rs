pub mod part1;
pub mod part2;

fn main() {
    let part1 = crate::part1::run();
    let part2 = crate::part2::run();

    println!();
    println!("Day 1, Part 1: {}", part1.unwrap());
    println!("Day 1, Part 2: {}", part2.unwrap());
}
