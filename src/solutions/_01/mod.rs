use std::collections::HashMap;

use super::utils::parse_file_line_by_line;

pub(crate) fn part_1(path: &str) -> i32 {
    let (mut left, mut right) = get_vecs(path);
    left.sort();
    right.sort();
    left.into_iter()
        .zip(right)
        .fold(0, |acc, (l, r)| acc + l.abs_diff(r) as i32)
}

pub(crate) fn part_2(path: &str) -> i32 {
    let (left, right) = get_vecs(path);
    let mut occurences: HashMap<i32, i32> = HashMap::new();
    for r in right {
        *occurences.entry(r).or_default() += 1;
    }
    left.into_iter()
        .fold(0, |acc, l| acc + occurences.get(&l).unwrap_or(&0) * l)
}

fn get_vecs(path: &str) -> (Vec<i32>, Vec<i32>) {
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
    (left, right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01_part_1() {
        assert_eq!(part_1("src/solutions/_01/_01.test.txt"), 11)
    }

    #[test]
    fn test_01_part_2() {
        assert_eq!(part_2("src/solutions/_01/_01.test.txt"), 31)
    }
}
