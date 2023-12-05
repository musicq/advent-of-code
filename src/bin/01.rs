advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let sum = input.lines().fold(0u32, |mut acc, cur| {
        let mut first: Option<u32> = None;
        let mut last: Option<u32> = None;

        for c in cur.chars() {
            if c.is_numeric() {
                if first.is_none() {
                    first = Some(c.to_digit(10).unwrap());
                } else {
                    last = Some(c.to_digit(10).unwrap());
                }
            }
        }
        if last.is_none() {
            last = first;
        }

        acc += (first.unwrap() * 10) + last.unwrap();

        acc
    });

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0u32;

    for line in input.lines() {
        let mut first: Option<u32> = None;
        let mut last: Option<u32> = None;

        for (i, c) in line.chars().enumerate() {
            let mut n: Option<u32> = None;
            if c.is_numeric() {
                n = c.to_digit(10);
            }

            match c {
                'o' => {
                    if line.get(i + 1..i + 3) == Some("ne") {
                        n = Some(1);
                    }
                }
                't' => {
                    if line.get(i + 1..i + 3) == Some("wo") {
                        n = Some(2);
                    } else if line.get(i + 1..i + 5) == Some("hree") {
                        n = Some(3);
                    }
                }
                'f' => {
                    if line.get(i + 1..i + 4) == Some("our") {
                        n = Some(4);
                    } else if line.get(i + 1..i + 4) == Some("ive") {
                        n = Some(5);
                    }
                }
                's' => {
                    if line.get(i + 1..i + 3) == Some("ix") {
                        n = Some(6);
                    } else if line.get(i + 1..i + 5) == Some("even") {
                        n = Some(7);
                    }
                }
                'e' => {
                    if line.get(i + 1..i + 5) == Some("ight") {
                        n = Some(8);
                    }
                }
                'n' => {
                    if line.get(i + 1..i + 4) == Some("ine") {
                        n = Some(9);
                    }
                }
                'z' => {
                    if line.get(i + 1..i + 4) == Some("ero") {
                        n = Some(0);
                    }
                }
                _ => {}
            }

            if n.is_none() {
                continue;
            }

            if first.is_none() {
                first = n;
            } else {
                last = n;
            }
        }

        if last.is_none() {
            last = first;
        }

        sum += (first.unwrap() * 10) + last.unwrap();
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(56108));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55652));
    }
}
