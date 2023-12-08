use std::collections::HashSet;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let mut r = 0;
    let mut set = HashSet::new();

    for line in input.lines() {
        let mut power = 0;
        let parts = &line[10..].split(" | ").collect::<Vec<&str>>();
        parts[1].split(' ').for_each(|s| {
            set.insert(s);
        });
        let nums = parts[0].split(' ').collect::<Vec<&str>>();

        for n in nums {
            if set.contains(n) {
                power += 1;
            }
        }

        if power != 0 {
            r += (2u32).pow(power - 1);
        }

        set.clear();
    }

    Some(r)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines().collect::<Vec<&str>>();
    let lines_len = lines.len();
    let mut r: Vec<u32> = vec![0; lines_len];
    let mut set = HashSet::new();

    for (i, line) in lines.iter().enumerate() {
        let mut occurences = 0;
        let parts = &line[10..].split(" | ").collect::<Vec<&str>>();
        parts[1].split(' ').for_each(|s| {
            set.insert(s);
        });
        let nums = parts[0].split(' ').collect::<Vec<&str>>();

        for n in nums {
            if n.trim().is_empty() {
                continue;
            }

            if set.contains(n) {
                occurences += 1;
            }
        }

        r[i] = occurences;

        set.clear()
    }

    let mut s: Vec<u32> = vec![1; lines_len];
    for (i, &v) in r.iter().enumerate() {
        for j in (i + 1)..=(i + v as usize) {
            s[j] += s[i] * 1;
        }
    }

    Some(s.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(59785));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5920640));
    }
}
