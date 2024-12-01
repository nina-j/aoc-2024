mod day01;

fn main() {
    let input_text = include_str!("day01/input");
    println!("Part 01: {}", day01::part01(input_text));
    println!("Part 02: {}", day01::part02(input_text));
}
