use std::iter::zip;

fn input_to_lists(input: &str) -> (Vec<i32>, Vec<i32>) {
    let result: Vec<_> = input
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .collect();
    let mut list0: Vec<i32> = Vec::new();
    let mut list1: Vec<i32> = Vec::new();
    for row in result.iter() {
        list0.push(row[0].parse::<i32>().unwrap());
        list1.push(row[1].parse::<i32>().unwrap());
    }
    (list0, list1)
}

pub fn part01(input_text: &str) -> i32 {
    let (mut list0, mut list1) = input_to_lists(input_text);
    list0.sort();
    list1.sort();
    let mut sum = 0;
    for elem in zip(list0, list1) {
        sum += i32::abs(elem.1 - elem.0);
    }
    sum
}

pub fn part02(input_text: &str) -> i32 {
    let (list0, list1) = input_to_lists(input_text);
    let mut sum = 0;
    for elem in list0 {
        sum += elem * list1.iter().filter(|&x| x == &elem).count() as i32;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_01() {
        let test01 = include_str!("./test");
        assert_eq!(11, part01(test01));
    }

    #[test]
    fn test_part_02() {
        let test02 = include_str!("./test");
        assert_eq!(31, part02(test02));
    }
}
