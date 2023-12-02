use std::convert::From;
use std::fs;
#[derive(Debug)]
struct Config {
    blue: u8,
    red: u8,
    green: u8,
}
impl From<&str> for Config {
    fn from(value: &str) -> Self {
        let mut config = Config {
            blue: 0,
            red: 0,
            green: 0,
        };

        for part in value.split(", ").into_iter() {
            if let Some((count_str, color)) = part.split_once(" ") {
                println!("color: {} count: {}", color, count_str);
                let count = count_str.parse::<u8>().expect("Failed to parse count str");
                match color {
                    "blue" => config.blue = count,
                    "red" => config.red = count,
                    "green" => config.green = count,
                    _ => panic!("Unexpected color"),
                };
            }
        }
        config
    }
}

struct Game {
    id: u8,
    plays: String,
}
impl From<&str> for Game {
    fn from(value: &str) -> Self {
        let (game_id_str, configs_str) = value.split_once(": ").expect("Invalid game input");
        Game {
            id: game_id_str
                .split_once(" ")
                .expect("Invalid game id")
                .1
                .parse()
                .expect("Failed to parse game id"),
            plays: configs_str.to_owned(),
        }
    }
}


fn main() {

    let config = Config {
        red: 12,
        green: 13,
        blue: 14,
    };
    let result: u64 = full(&fs::read_to_string("./input.txt").expect("Failed to read file"), &config);
    println!("Result: {}", result);
    let result: u64 = power(&fs::read_to_string("./input.txt").expect("Failed to read file"));
    println!("Result: {}", result);
}


fn is_game_possible(input: &str, bag_config: &Config) -> bool {
    !input
        .split("; ")
        .into_iter()
        .map(|x| Config::from(x))
        .inspect(|x| println!("{:#?}", x))
        .any(|x| x.blue > bag_config.blue || x.red > bag_config.red || x.green > bag_config.green)
}

fn full(input: &str, bag_config: &Config) -> u64 {
    input
        .split("\n")
        .filter(|x| !x.is_empty())
        .into_iter()
        .map(|x| Game::from(x))
        .filter(|x| is_game_possible(&x.plays, &bag_config))
        .map(|x| x.id as u64)
        .sum()
}

fn power(input: &str) -> u64 {
    input
        .split("\n")
        .filter(|x| !x.is_empty())
        .into_iter()
        .map(|x| Game::from(x))
        .map(|x| game_power(&x.plays))
        .sum()
}
fn game_power(input: &str) -> u64 {

    let mut max = Config {
        blue: 0,
        red: 0,
        green: 0
    };
    input
        .split("; ")
        .into_iter()
        .map(|x| Config::from(x))
        .for_each(|x| {
            if x.blue > max.blue {
                max.blue = x.blue;
            }
            if x.red > max.red {
                max.red = x.red;
            }
            if x.green > max.green {
                max.green = x.green;
            }
        });
    max.blue as u64 * max.red as u64 * max.green as u64


}

#[test]
fn test_is_game_possible() {
    assert!(!is_game_possible(
        "3 blue; 24 red, 5 green; 2 red",
        &Config {
            blue: 3,
            green: 6,
            red: 7
        }
    ));
    assert!(is_game_possible(
        "3 blue; 7 red, 5 green; 2 red",
        &Config {
            blue: 3,
            green: 6,
            red: 7
        }
    ));
}

#[test]
fn test_full() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    let config = Config {
        red: 12,
        green: 13,
        blue: 14,
    };
    assert_eq!(full(input, &config), 8);
}
#[test]
fn test_power() {

    let input ="Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    assert_eq!(power(input), 2286);
}
