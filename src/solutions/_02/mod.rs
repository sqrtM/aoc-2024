use super::utils::parse_file_line_by_line;

#[derive(Debug)]
enum Direction {
    Inc,
    Dec,
    None,
}

#[derive(Debug)]
struct Level {
    value: i32,
    valid: bool,
    dampener: bool,
    direction: Direction,
}

impl Level {
    fn new(with_dampener: bool, direction: Direction) -> Self {
        Level {
            value: -1,
            valid: true,
            dampener: with_dampener,
            direction,
        }
    }

    fn increment(mut self, next: i32) -> Self {
        if self.value == -1 {
            self.value = next;
            return self;
        }
        if !self.valid {
            return self;
        }
        println!("{:?}", self);
        match self.direction {
            Direction::Inc => {
                if next <= self.value || next.abs_diff(self.value) > 3 {
                    if self.dampener {
                        println!("DAMPENED {:?}", next);
                        self.dampener = false;
                        return self;
                    }
                    self.valid = false;
                }
                self.value = next;

                self
            }
            Direction::Dec => {
                if next >= self.value || next.abs_diff(self.value) > 3 {
                    if self.dampener {
                        println!("DAMPENED {:?}", next);
                        self.dampener = false;
                        return self;
                    }
                    self.valid = false;
                }
                self.value = next;

                self
            }
            Direction::None => {
                if next.abs_diff(self.value) > 3 || next.abs_diff(self.value) == 0 {
                    if self.dampener {
                        println!("DAMPENED {:?}", next);
                        self.dampener = false;
                        return self;
                    } else {
                        self.value = next;
                        self.valid = false
                    }
                }

                if next > self.value {
                    self.direction = Direction::Inc;
                }
                if next < self.value {
                    self.direction = Direction::Dec;
                }
                self.value = next;
                self
            }
        }
    }
}

pub(crate) fn part_1(path: &str) -> i32 {
    let mut valid_lines = 0;
    parse_file_line_by_line(path, |line| {
        if line
            .split_whitespace()
            .map(|str| str.parse::<i32>().unwrap())
            .fold(Level::new(false, Direction::None), |acc, next| {
                acc.increment(next)
            })
            .valid
        {
            valid_lines += 1;
        }
    })
    .unwrap();
    valid_lines
}

pub(crate) fn part_2(path: &str) -> i32 {
    let mut valid_lines = 0;
    parse_file_line_by_line(path, |line| {
        println!("------{:?}----------", line);
        if line
            .split_whitespace()
            .map(|str| str.parse::<i32>().unwrap())
            .fold(Level::new(true, Direction::None), |acc, next| {
                acc.increment(next)
            })
            .valid
        {
            valid_lines += 1;
            println!("-- good !");
        } else {
            println!("-- bad !");
        }
    })
    .unwrap();
    valid_lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_02_part_1() {
        assert_eq!(part_1("src/solutions/_02/_02.test.txt"), 2)
    }

    #[test]
    fn test_02_part_2() {
        assert_eq!(part_2("src/solutions/_02/_02.test.txt"), 4)
    }
}
