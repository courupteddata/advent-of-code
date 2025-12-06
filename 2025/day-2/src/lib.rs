use std::collections::{HashSet};
use std::str::Chars;

const MIDDLE_SEPERATOR: char = '-';
const END_SEPERATOR: char = ',';

pub struct RepetitionFinder <'a>{
    iter: Chars<'a>,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct RangeSegment {
    start: u64,
    end: u64,
}

impl<'a> RepetitionFinder<'a> {
    pub fn new(input: &'a str) -> RepetitionFinder<'a> {
        RepetitionFinder {iter: input.chars().clone()}
    }
}

impl<'a> Iterator for RepetitionFinder<'a> {
    type Item = RangeSegment;

    fn next(&mut self) -> Option<Self::Item> {
        let mut start = "".to_string();
        let mut end = "".to_string();

        let mut found_middle = false;

        for character in self.iter.by_ref() {
            if character == MIDDLE_SEPERATOR {
                found_middle = true;
            } else if character == END_SEPERATOR {
                return Option::from(RangeSegment { start: start.parse().unwrap(), end: end.parse().unwrap() });
            } else if character.is_digit(10) {
                if found_middle {
                    end.push(character);
                } else {
                    start.push(character);
                }
            }
        }
        if found_middle {
            return Option::from(RangeSegment { start: start.parse().unwrap(), end: end.parse().unwrap() });
        }

        None
    }
}

impl RangeSegment {
    pub fn process_range(&self) -> u64 {
        let mut total = 0;

        for entry in self.start..=self.end {
            let entry_string = entry.to_string();
            if entry_string.len() % 2 == 0 {
                // Even are the only ones to look at
                let beginning = entry_string.get(0..entry_string.len()/2).unwrap();
                let ending = entry_string.get(entry_string.len()/2..).unwrap();
                // .chars().rev().collect::<String>(); // to reverse
                if beginning == ending {
                    total += entry;
                }
            }
        }

        total
    }

    pub fn process_range_advanced(&self) -> u64 {
        let mut total = 0;
        // WARNING: This is a very suboptimal approach
        for entry in self.start..=self.end {
            let entry_string = entry.to_string();

            // substrings, if the substring is a multiple of the main string
            //
            let mut substrings: HashSet<String> = HashSet::new();

            for char in entry_string.chars() {
                let mut new_substrings = HashSet::new();
                // maybe something with hash maps
                // counted set
                new_substrings.insert(char.to_string());
                for substring in substrings.iter() {
                    let appended_substring = format!("{}{}", substring.clone(), char);
                    new_substrings.insert(appended_substring);
                }
                substrings.extend(new_substrings)
            }

            for substring in substrings.iter() {
                let (division, remainder) = (entry_string.len() / substring.len(), entry_string.len() % substring.len());
                if remainder == 0 {
                    // We have a substring multiple
                    if entry_string == substring.repeat(division) && division > 1 {
                        total += entry;
                        break;
                    }
                }
            }

        }

        total
    }
}


#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        let example_input = "11-22,95-115,2121212118-2121212124";

        let mut repetition_finder = RepetitionFinder::new(example_input);

        assert_eq!(RangeSegment{start: 11, end: 22}, repetition_finder.next().unwrap());
        assert_eq!(RangeSegment{start: 11, end: 22}.process_range(), 33);
        assert_eq!(RangeSegment{start: 11, end: 22}.process_range_advanced(), 33);
        assert_eq!(RangeSegment{start: 95, end: 115}, repetition_finder.next().unwrap());
        assert_eq!(RangeSegment{start: 95, end: 115}.process_range(), 99);
        assert_eq!(RangeSegment{start: 95, end: 115}.process_range_advanced(), 210);
        assert_eq!(RangeSegment{start: 2121212118, end: 2121212124}, repetition_finder.next().unwrap());
        assert_eq!(RangeSegment{start: 2121212118, end: 2121212124}.process_range(), 0);
        assert_eq!(RangeSegment{start: 2121212118, end: 2121212124}.process_range_advanced(), 2121212121);
    }
}
