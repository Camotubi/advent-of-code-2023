use std::collections::HashMap;
use std::fs;

fn main() {
    let result: u64 = full(&fs::read_to_string("./input.txt").expect("Failed to read file"));
    println!("Result: {}", result);
}

fn full(input: &str) -> u64 {
    let atlas: HashMap<&str, &str> = HashMap::from([
        ("zero", "0"),
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);
    input
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| preprocess(&x, &atlas))
        .map(|x| decode(&x))
        .sum()
}

fn decode(input: &str) -> u64 {
    let mut first: Option<char> = None;
    let mut second: Option<char> = None;
    for char in input.chars() {
        let number = char.to_digit(10);
        if number.is_none() {
            continue;
        }

        if first.is_none() {
            first = Some(char);
            continue;
        }
        second = Some(char);
    }
    if second.is_none() {
        second = first;
    }

    let result = [
        first.expect("Failed to get first"),
        second.expect("Failed to get second"),
    ]
    .into_iter()
    .collect::<String>()
    .parse()
    .unwrap();
    println!("result:{}", result);
    result
}

fn preprocess(input: &str, atlas: &HashMap<&str, &str>) -> String {
    let mut result = "".to_owned();
    let mut b = 0;
    while b < input.len() {
        let mut e = b;
        while e < input.len() {
            let word = input.chars().skip(b).take(e - b + 1).collect::<String>();
            if let Some(replace) = atlas.get(&*word) {
                result = [&result, *replace].into_iter().collect();
            } else if word.len() == 1 {
                if let Ok(_) = word.parse::<u8>() {
                    result = [result, word].into_iter().collect();
                }
            }
            e += 1;
        }
        b += 1;
    }
    print!("input: {} preprocess {} ", input, result);
    result
}

#[test]
fn test() {
    assert_eq!(decode("1abc2"), 12);
    assert_eq!(decode("pqr3stu8vwx"), 38);
    assert_eq!(decode("a1b2c3d4e5f"), 15);
    assert_eq!(decode("treb7uchet"), 77);
}

#[test]
fn test_preprocess() {
    let atlas: HashMap<&str, &str> = HashMap::from([
        ("zero", "0"),
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ]);
    assert_eq!(preprocess("two1nine", &atlas), "219");
    assert_eq!(preprocess("eightwothree", &atlas), "823");
    assert_eq!(preprocess("abcone2threexyz", &atlas), "123");
    assert_eq!(preprocess("xtwone3four", &atlas), "2134");
    assert_eq!(preprocess("4nineeightseven2", &atlas), "49872");
    assert_eq!(preprocess("zoneight234", &atlas), "18234");
    assert_eq!(preprocess("7pqrstsixteen", &atlas), "76");
}

#[test]
fn test_all() {
    let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    assert_eq!(full(input), 281);
}
