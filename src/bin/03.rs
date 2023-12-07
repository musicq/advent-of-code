advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines().collect::<Vec<&str>>();
    let lines_len = lines.len();
    let mut line_idx = 0;
    let mut sum = 0;

    while line_idx < lines_len {
        let mut char_idx = 0;
        let line = lines[line_idx];
        let chars_len = line.len();
        let chars = line.chars().collect::<Vec<char>>();

        while char_idx < chars_len {
            if !chars[char_idx].is_numeric() {
                char_idx += 1;
                continue;
            }

            // get the num
            let mut num_str = String::from(chars[char_idx]);
            let mut last_char_idx = char_idx + 1;
            while last_char_idx < chars_len && chars[last_char_idx].is_numeric() {
                num_str.push(chars[last_char_idx]);
                last_char_idx += 1;
            }

            // check if num is qualified
            let prev_line = if line_idx == 0 { 0 } else { line_idx - 1 };
            let next_line = if line_idx + 1 >= lines_len {
                line_idx
            } else {
                line_idx + 1
            };
            let start = if char_idx == 0 { 0 } else { char_idx - 1 };
            let end = if last_char_idx >= chars_len {
                last_char_idx - 1
            } else {
                last_char_idx
            };

            for l in prev_line..=next_line {
                let line = lines[l].chars().collect::<Vec<char>>();
                let mut is_qualified = false;
                for c in start..=end {
                    if !line[c].is_numeric() && line[c] != '.' {
                        is_qualified = true;
                        sum += num_str.parse::<u32>().unwrap();
                        break;
                    }
                }

                if is_qualified {
                    break;
                }
            }

            char_idx = last_char_idx;
        }

        line_idx += 1;
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines().collect::<Vec<&str>>();
    let lines_len = lines.len();
    let mut line_idx = 0;
    let mut sum = 0;

    while line_idx < lines_len {
        let mut char_idx = 0;
        let line = lines[line_idx];
        let chars_len = line.len();
        let chars = line.chars().collect::<Vec<char>>();

        while char_idx < chars_len {
            if chars[char_idx] != '*' {
                char_idx += 1;
                continue;
            }

            let prev_line = if line_idx == 0 { 0 } else { line_idx - 1 };
            let next_line = if line_idx + 1 >= lines_len {
                line_idx
            } else {
                line_idx + 1
            };
            let start = if char_idx == 0 { 0 } else { char_idx - 1 };
            let end = if char_idx + 1 >= chars_len {
                char_idx
            } else {
                char_idx + 1
            };

            let mut nums = vec![];
            let mut is_qualified = true;
            for l in prev_line..=next_line {
                let line = lines[l].chars().collect::<Vec<char>>();
                let mut c = start;

                while c <= end {
                    if line[c].is_numeric() {
                        if nums.len() > 2 {
                            is_qualified = false;
                            break;
                        }
                        let mut num_str = String::from(line[c]);
                        let mut f = (c - 1) as i32;
                        let mut e = c + 1;
                        while f >= 0 && line[f as usize].is_numeric() {
                            num_str.insert(0, line[f as usize]);
                            f -= 1;
                        }
                        while e < chars_len && line[e].is_numeric() {
                            num_str.push(line[e]);
                            e += 1;
                        }

                        nums.push(num_str);

                        c = e;
                        continue;
                    }

                    c += 1;
                }

                if !is_qualified {
                    break;
                }
            }

            if is_qualified && nums.len() == 2 {
                let mut v = 1;
                for n in nums {
                    v *= n.parse::<u32>().unwrap();
                }
                sum += v;
            }

            char_idx += 1;
        }

        line_idx += 1;
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(529618));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(77509019));
    }
}
