mod day01;
mod day02;

fn main() {
    println!("Part 01: {}", day01::part01("./src/day01/input"));
    println!("Part 02: {}", day01::part02(include_str!("day01/input")));
}
