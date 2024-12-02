use super::utils::parse_file_line_by_line;

pub(crate) fn solve(path: &str) -> i32 {
    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];
    parse_file_line_by_line(path, |line| {
        line.split_whitespace().enumerate().for_each(|(i, num)| {
            if i == 0 {
                left.push(num.parse::<i32>().unwrap());
            } else {
                right.push(num.parse::<i32>().unwrap());
            }
        });
    })
    .unwrap();
    left.sort();
    right.sort();
    left.into_iter().zip(right).fold(0, |acc, (l, r)| {
        println!("{:?}, {:?}", l, r);
        acc + l.abs_diff(r) as i32
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01_part_1() {
        assert_eq!(solve("src/solutions/_01/_01.test.txt"), 11)
    }
}
