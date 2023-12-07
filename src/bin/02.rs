advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let pool = Game(12, 13, 14);
    let mut sum = 0;

    for (i, line) in input.lines().enumerate() {
        let mut satisfied = true;
        for set in parse_game_set(line) {
            if set.0 > pool.0 || set.1 > pool.1 || set.2 > pool.2 {
                satisfied = false;
                break;
            }
        }

        if satisfied {
            sum += i + 1;
        }
    }

    Some(sum as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;

    for line in input.lines() {
        let mut game_cubs = Game::new();
        for set in parse_game_set(line) {
            game_cubs.0 = game_cubs.0.max(set.0);
            game_cubs.1 = game_cubs.1.max(set.1);
            game_cubs.2 = game_cubs.2.max(set.2);
        }

        sum += game_cubs.0 * game_cubs.1 * game_cubs.2;
    }

    Some(sum as u32)
}

#[derive(Debug, PartialEq, Eq)]
struct Game(u32, u32, u32); // r, g, b

impl Game {
    fn new() -> Game {
        Game(0, 0, 0)
    }
}

fn parse_game_set(game_set: &str) -> Vec<Game> {
    let mut r: Vec<Game> = vec![];
    let mut chars = game_set.chars();

    while let Some(c) = chars.next() {
        if c == ':' {
            break;
        }
    }

    let mut set = Game::new();

    while let Some(c) = chars.next() {
        if c == ';' {
            r.push(set);
            set = Game::new();
            continue;
        }

        if c.is_numeric() {
            let mut num_str = String::from(c);
            while let Some(s) = chars.next() {
                if s.is_numeric() {
                    num_str.push(s);
                    continue;
                }

                match s {
                    'r' => {
                        set.0 = num_str.parse::<u32>().unwrap();
                        break;
                    }
                    'g' => {
                        set.1 = num_str.parse::<u32>().unwrap();
                        break;
                    }
                    'b' => {
                        set.2 = num_str.parse::<u32>().unwrap();
                        break;
                    }
                    _ => continue,
                }
            }

            continue;
        }
    }

    if set != Game::new() {
        r.push(set)
    }

    r
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2879));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65122));
    }

    #[test]
    fn test_parse_game_set() {
        let str = "Game 1: 4 red, 18 green, 15 blue; 17 green, 18 blue, 9 red; 8 red, 14 green, 6 blue; 14 green, 12 blue, 2 red";
        let result = parse_game_set(str);

        assert_eq!(
            result,
            vec![
                Game(4, 18, 15),
                Game(9, 17, 18,),
                Game(8, 14, 6,),
                Game(2, 14, 12,),
            ]
        )
    }
}
