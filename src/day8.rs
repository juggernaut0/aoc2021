use std::str::FromStr;
use crate::util::parse_lines;

pub struct Solution;

impl crate::Solution for Solution {
   fn solve_1(&self, input: String) -> String {
       parse_lines(&input)
           .map(|it: Entry| {
               it.digits.iter()
                   .filter(|d| {
                       let len = d.len();
                       len == 2 || len == 3 || len == 4 || len == 7
                   })
                   .count()
           })
           .sum::<usize>()
           .to_string()
   }

   fn solve_2(&self, input: String) -> String {
       parse_lines(&input)
           .map(|it: Entry| {
               let mapping = mapping(&it);
               let s: String = it.digits.into_iter().map(|d| get_digit(&d, &mapping)).collect();
               s.parse::<u32>().unwrap()
           })
           .sum::<u32>()
           .to_string()
   }
}

/*
 aaaa0
b1   c2
b    c
 dddd3
e4   f5
e    f
 gggg6
 */

// outputs a String that is the mapping of entry signals to actual segments, ordered by the above diagram
fn mapping(entry: &Entry) -> Vec<char> {
    let mut result = vec![' '; 7];

    let one = entry.combs.iter().find(|it| it.len() == 2).unwrap();
    let seven = entry.combs.iter().find(|it| it.len() == 3).unwrap();
    // r[0] = the letter that is in seven but not one
    result[0] = find_remaining(seven, one, &result);

    let four = entry.combs.iter().find(|it| it.len() == 4).unwrap();
    let nine = entry.combs.iter()
        .find(|it| {
            let len_6 = it.len() == 6;
            let has_all_of_four = four.chars().all(|c| it.contains(c));
            let has_0 = it.chars().any(|c| c == result[0]);
            len_6 && has_all_of_four && has_0
        })
        .unwrap();
    // r[6] = the letter that is in nine but not in four
    result[6] = find_remaining(nine, four, &result);

    let three = entry.combs.iter()
        .find(|it| {
            let has_0 = it.chars().any(|c| c == result[0]);
            let has_6 = it.chars().any(|c| c == result[6]);
            let has_all_of_one = one.chars().all(|c| it.contains(c));
            it.len() == 5 && has_0 && has_6 & has_all_of_one
        })
        .unwrap();
    // r[3] = the letter than is in three but not in one
    result[3] = find_remaining(three, one, &result);

    // r[1] = the letter that is in four but not in one
    result[1] = find_remaining(four, one, &result);

    // r[5] = the letter that is in five
    let five = entry.combs.iter()
        .find(|it| {
            it.len() == 5 && result.iter().copied().all(|c| c == ' ' || it.contains(c))
        })
        .unwrap();
    result[5] = find_remaining(five, "", &result);

    // r[4] = the letter that is in six
    let six = entry.combs.iter()
        .find(|it| {
            it.len() == 6 && it != &nine && result.iter().copied().all(|c| c == ' ' || it.contains(c))
        })
        .unwrap();
    result[4] = find_remaining(six, "", &result);

    result[2] = find_remaining(one, "", &result);

    assert!(result.iter().copied().all(|c| c != ' '));

    result
}

fn find_remaining(searching: &str, not: &str, taken: &[char]) -> char {
    log::debug!("searching {} not {} taken {:?}", searching, not, taken);
    assert!(searching.len() <= not.len() + taken.iter().filter(|c| **c != ' ').count() + 1, "cannot search");
    searching.chars().find(|c| !(not.contains(*c) || taken.contains(c))).unwrap()
}

fn get_digit(signals: &str, mapping: &[char]) -> char {
    let on = [
        signals.contains(mapping[0]),
        signals.contains(mapping[1]),
        signals.contains(mapping[2]),
        signals.contains(mapping[3]),
        signals.contains(mapping[4]),
        signals.contains(mapping[5]),
        signals.contains(mapping[6]),
    ];

    match on {
        [true, true, true, false, true, true, true] => '0',
        [false, false, true, false, false, true, false] => '1',
        [true, false, true, true, true, false, true] => '2',
        [true, false, true, true, false, true, true] => '3',
        [false, true, true, true, false, true, false] => '4',
        [true, true, false, true, false, true, true] => '5',
        [true, true, false, true, true, true, true] => '6',
        [true, false, true, false, false, true, false] => '7',
        [true, true, true, true, true, true, true] => '8',
        [true, true, true, true, false, true, true] => '9',
        _ => panic!("unrecognized format: {:?}", on),
    }
}

struct Entry {
    combs: Vec<String>,
    digits: Vec<String>,
}

impl FromStr for Entry {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace().map(|it| it.to_string());
        let combs = vec![
            split.next().unwrap(),
            split.next().unwrap(),
            split.next().unwrap(),
            split.next().unwrap(),
            split.next().unwrap(),
            split.next().unwrap(),
            split.next().unwrap(),
            split.next().unwrap(),
            split.next().unwrap(),
            split.next().unwrap(),
        ];
        split.next().unwrap(); // skip pipe
        let digits = vec![
            split.next().unwrap(),
            split.next().unwrap(),
            split.next().unwrap(),
            split.next().unwrap(),
        ];
        Ok(Entry { combs, digits })
    }
}
