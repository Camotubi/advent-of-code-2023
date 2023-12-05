use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let result: u64 = process(&fs::read_to_string("./input.txt").expect("Failed to read file"));
    println!("Result: {}", result);
}
#[derive(Hash, PartialEq, Eq, Debug)]
struct NumFound {
    num: u64,
    line: u64,
    line_position: u64,
}

fn process(input: &str) -> u64 {
    let lines: Vec<_> = input.split("\n").filter(|x| !x.is_empty()).collect();
    let mut map: HashMap<(u64, u64), HashSet<NumFound>> = HashMap::new();
    for win in lines.windows(2).enumerate() {
        find_matched_nums(win.1, win.0 as u64, &mut map);
    }
    map.into_values()
        .inspect(|x| println!("hash set: {:#?}", x))
        .filter_map(|x| match x.len() {
            2 => Some(x.into_iter().map(|n| n.num).product::<u64>()),
            _ => None,
        })
        .into_iter()
        .sum()
}

#[test]
fn test_process() {
    let input = "
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
    assert_eq!(process(&input), 467835);
}

fn find_matched_nums(
    input: &[&str],
    window_index: u64,
    found: &mut HashMap<(u64, u64), HashSet<NumFound>>,
) {
    let mut specials: (Vec<(u64, u64)>, Vec<(u64, u64)>) = (
        find_specials(input[0])
            .into_iter()
            .map(|x| (window_index, x))
            .collect(),
        find_specials(input[1])
            .into_iter()
            .map(|x| (window_index + 1, x))
            .collect(),
    );
    specials.0.extend(specials.1);
    let specials = specials.0;
    let nums = (find_nums(input[0]), find_nums(input[1]));
    for special in specials.into_iter() {
        let entry = found.entry(special).or_insert(HashSet::new());
        for num in &nums.0 {
            if is_num_within(special.1, num.0, num.1.len() as u64) {
                entry.insert(NumFound {
                    num: num.1.parse().expect("Wrong number"),
                    line: window_index,
                    line_position: num.0,
                });
            }
        }
        for num in &nums.1 {
            if is_num_within(special.1, num.0, num.1.len() as u64) {
                entry.insert(NumFound {
                    num: num.1.parse().expect("Wrong number"),
                    line: window_index + 1,
                    line_position: num.0,
                });
            }
        }
    }
}

/*
#[test]
fn test_find_matched_nums() {
    let input = ["467..114..", "...*......"];
    assert!(find_matched_nums(&input, 2).contains(&NumFound {
        num: 467,
        line: 2,
        line_position: 0
    }));

    let input = [".....+.588", "..592....."];
    let result = find_matched_nums(&input, 3);
    assert!(result.contains(&NumFound {
        num: 592,
        line: 4,
        line_position: 2
    }));
}
*/

fn is_num_within(special_pos: u64, num_pos: u64, num_digit_len: u64) -> bool {
    let min_pos = if num_pos == 0 { 0 } else { num_pos - 1 };
    let max_pos = num_pos + num_digit_len /*+ 1 */;
    special_pos >= min_pos && special_pos <= max_pos
    // .456.
    // ..*..
}
#[test]
fn test_is_num_within() {
    assert!(!is_num_within(5, 7, 3));
    assert!(!is_num_within(85, 81, 3));
}

fn find_specials(line: &str) -> Vec<u64> {
    line.chars()
        .into_iter()
        .enumerate()
        .filter_map(|x| {
            if x.1 == '*' {
                return Some(x.0 as u64);
            }
            None
        })
        .collect()
}

#[test]
fn test_find_specials() {
    assert!(find_specials("467..114..").is_empty());
    let result = find_specials("...*.../..#...6");

    assert_eq!(result.len(), 1);
    assert!(result.contains(&3));
}

fn find_nums(line: &str) -> Vec<(u64, String)> {
    let mut buffer: Vec<char> = Vec::new();
    let mut nums: Vec<(u64, String)> = Vec::new();
    for character in line.chars().into_iter().enumerate() {
        if character.1.is_digit(10) {
            buffer.push(character.1);
        } else if !buffer.is_empty() {
            // println!("{}", character.0);
            let pos = if buffer.len() == 1 {
                // 1 length char
                (character.0 - 1) as u64
            } else {
                let first_char_pos_offset = if buffer.len() > 1 { buffer.len() } else { 0 };
                (character.0 - first_char_pos_offset) as u64
            };
            nums.push((pos, buffer.clone().into_iter().collect()));
            buffer.clear();
        }
    }
    if !buffer.is_empty() {
        nums.push((
            (line.len() - buffer.len()) as u64,
            buffer.clone().into_iter().collect(),
        ));
    }
    nums
}

#[test]
fn test_find_nums() {
    let result = find_nums("467..114..");

    assert_eq!(result.len(), 2);
    assert!(result.contains(&(0, "467".to_owned())));
    assert!(result.contains(&(5, "114".to_owned())));
    let result = find_nums(".....+.588");
    assert!(result.contains(&(7, "588".to_owned())));

    let result = find_nums("11...");
    println!("{:#?}", result);
    assert!(result.contains(&(0, "11".to_owned())));

    let result = find_nums(".1...");
    println!("{:#?}", result);
    assert!(result.contains(&(1, "1".to_owned())));

    let result = find_nums("....1");
    println!("{:#?}", result);
    assert!(result.contains(&(4, "1".to_owned())));

    let result = find_nums("...11");
    println!("{:#?}", result);
    assert!(result.contains(&(3, "11".to_owned())));
}
