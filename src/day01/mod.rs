use polars::prelude::*;

fn input_to_lf(input: &str, sep: u8) -> LazyFrame {
    let lf: LazyFrame = LazyCsvReader::new(input)
        .with_has_header(false)
        .with_separator(sep)
        .finish()
        .unwrap();
    lf
}

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

pub fn part01(input_path: &str) -> DataFrame {
    let lf = input_to_lf(input_path, b' ').drop(["column_2", "column_3"]);

    lf.select([
        col("column_1").sort(Default::default()).alias("list_0"),
        col("column_4").sort(Default::default()).alias("list_1"),
    ])
    .select([(col("list_0") - col("list_1")).alias("diff")])
    .select([col("diff").abs().sum().alias("diff_sum")])
    .collect()
    .unwrap()
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
    use polars::df;
    #[test]
    fn test_input_to_df() {
        let expected = df![
            "a" => &[3, 4, 2, 1, 3, 3],
            "b" => &[4, 3, 5, 3 , 9, 3]
        ];
        assert_eq!(
            expected.unwrap(),
            input_to_lf("./src/day01/test", b' ')
                .drop(["column_2", "column_3"])
                .collect()
                .unwrap()
        );
    }

    #[test]
    fn test_part_01() {
        let expected = df!["diff_sum" => [11]];
        assert_eq!(expected.unwrap(), part01("./src/day01/test"));
    }

    #[test]
    fn test_part_02() {
        assert_eq!(31, part02(include_str!("./test")));
    }
}
