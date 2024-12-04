use regex::Regex;

use super::utils::parse_file_line_by_line;

pub(crate) fn part_1(path: &str) -> i32 {
    let mul_re = Regex::new(r"mul\((\d*),(\d*)\)").unwrap();

    let mut total: i32 = 0;
    parse_file_line_by_line(path, |line| {
        mul_re.captures_iter(&line).for_each(|mat| {
            total += mat[1].parse::<i32>().unwrap() * mat[2].parse::<i32>().unwrap()
        });
    })
    .unwrap();
    total
}

pub(crate) fn part_2(path: &str) -> i32 {
    let mul_re = Regex::new(r"mul\((\d*),(\d*)\)").unwrap();
    let act_re = Regex::new(r"do(:?n't)?\(\)").unwrap();

    let mut total: i32 = 0;
    parse_file_line_by_line(path, |line| {
        act_re
            .captures_iter(&line)
            .for_each(|mat| println!("{:?}", &mat.get(0)));
        mul_re.captures_iter(&line).for_each(|mat| {
            total += mat[1].parse::<i32>().unwrap() * mat[2].parse::<i32>().unwrap()
        });
    })
    .unwrap();
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_03_part_1() {
        assert_eq!(part_1("src/solutions/_03/test.txt"), 48)
    }

    #[test]
    fn test_03_part_2() {
        assert_eq!(part_2("src/solutions/_03/test.txt"), 4)
    }
}
