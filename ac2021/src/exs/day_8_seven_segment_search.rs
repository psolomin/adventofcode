use std::collections::{BTreeSet, HashMap};
use std::iter::FromIterator;
use crate::exs::utils::read_lines_as_str_vector;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
struct DigitSymbolSeq {
    sequence: BTreeSet<char>,
}

impl DigitSymbolSeq {
    fn from_string(s: &String) -> DigitSymbolSeq {
        return DigitSymbolSeq { sequence: BTreeSet::from_iter(s.chars()) }
    }

    fn contains_all_bars_of(&self, other: &DigitSymbolSeq) -> bool {
        return self.sequence.is_superset(&other.sequence)
    }

    fn len(&self) -> usize {
        return self.sequence.len()
    }
}

#[derive(Debug)]
struct CoDec {
    decoding_map: HashMap<DigitSymbolSeq, i32>,
    encoding_map: HashMap<i32, DigitSymbolSeq>
}

impl CoDec {
    fn new() -> CoDec {
        return CoDec { decoding_map: HashMap::new(), encoding_map: HashMap::new() }
    }

    fn add_pair(&mut self, symbol_seq: DigitSymbolSeq, digit: i32) {
        self.decoding_map.insert(symbol_seq.clone(), digit);
        self.encoding_map.insert(digit, symbol_seq.clone());
    }

    fn encode(&self, digit: i32) -> Option<&DigitSymbolSeq> {
        return self.encoding_map.get(&digit)
    }

    fn decode(&self, symbol_seq: &DigitSymbolSeq) -> Option<&i32> {
        return self.decoding_map.get(symbol_seq)
    }

    fn has_symbol_seq(&self, symbol_seq: &DigitSymbolSeq) -> bool {
        return self.decoding_map.contains_key(symbol_seq)
    }
}

fn decode_line(
    line: &String,
    strategy: &dyn Fn(&Vec<String>, &Vec<String>) -> Vec<i32>
)-> Vec<i32> {
    let pattern_and_four_digits_output: Vec<Vec<String>> = line.split("|")
        .map(|x| x.trim())
        .map(|x| x.split(" ").map(|x| x.to_string()).collect())
        .collect();

    let pattern = pattern_and_four_digits_output.first().unwrap();
    let digits_output = pattern_and_four_digits_output.last().unwrap();
    return strategy(&pattern, &digits_output)
}

fn simple_strategy(_pattern: &Vec<String>, digits_output: &Vec<String>) -> Vec<i32> {
    return apply_chars_count_strategy(&digits_output)
}

fn full_search_strategy(pattern: &Vec<String>, digits_output: &Vec<String>) -> Vec<i32> {
    let preliminary_decoding_variant = apply_chars_count_strategy(&digits_output);
    if !preliminary_decoding_variant.contains(&-1) {
        return preliminary_decoding_variant
    }

    let (pattern_of_charsets, digits_output_charsets) =
        data_to_digits(pattern, digits_output);

    return  apply_deep_search_strategy(
        &pattern_of_charsets,
        &digits_output_charsets,
    )
}

fn data_segment_to_digits(data_segment: &Vec<String>) -> Vec<DigitSymbolSeq> {
    return data_segment.iter()
        .map(|s| DigitSymbolSeq::from_string(s)).collect::<Vec<DigitSymbolSeq>>()
}

fn data_to_digits(
    pattern: &Vec<String>, digits_output: &Vec<String>
) -> (Vec<DigitSymbolSeq>, Vec<DigitSymbolSeq>) {
    return (
        data_segment_to_digits(pattern),
        data_segment_to_digits(digits_output)
    )
}

fn decode_lines(lines: &Vec<String>, strategy: &dyn Fn(&Vec<String>, &Vec<String>) -> Vec<i32>) -> Vec<Vec<i32>> {
    let mut stats: Vec<Vec<i32>> = Vec::new();
    for line in lines {
        let output_digits = decode_line(line, strategy);
        stats.push(output_digits);
    }
    return stats
}

fn compute_result_part_1(stats: &Vec<Vec<i32>>) -> u64 {
    return stats.iter().flatten()
        .filter(|x| **x == 1 || **x == 4 || **x == 7 || **x == 8)
        .count() as u64
}

fn compute_result_part_2(stats: &Vec<Vec<i32>>) -> u64 {
    return stats.iter()
        .map(|seq| seq.iter().map(|x|x.to_string()).collect::<Vec<String>>().join(""))
        .map(|seq| seq.parse::<u64>().unwrap())
        .sum()
}

fn apply_chars_count_strategy(four_digits: &Vec<String>) -> Vec<i32> {
    four_digits.iter()
        .map(|x| x.len())
        .map(|x| length_match(x)).collect()
}

fn length_match(d: usize) -> i32 {
    return match d {
        2 => 1,
        3 => 7,
        4 => 4,
        7 => 8,
        _ => -1
    }
}

fn apply_deep_search_strategy(
    pattern: &Vec<DigitSymbolSeq>,
    digits_output: &Vec<DigitSymbolSeq>
) -> Vec<i32> {

    let mut codec = CoDec::new();

    for p in pattern.iter() {
        let maybe_digit = length_match(p.len());
        if maybe_digit != -1 {
            codec.add_pair(p.clone(), maybe_digit)
        }
    }

    // 3 is a combo of 5 bars which includes 1
    let three = pattern.iter()
        .filter(|x| {
            !codec.has_symbol_seq(x) &&
                x.len() == 5 &&
                x.contains_all_bars_of(codec.encode(1).unwrap())
        }).next().unwrap();

    codec.add_pair(three.clone(), 3);

    // 3 and 9 have only one bar which is different
    let nine = pattern.iter()
        .filter(|x| {
            !codec.has_symbol_seq(x) &&
            x.len() == 6 &&
                x.contains_all_bars_of(three)
        }).next().unwrap();

    codec.add_pair(nine.clone(), 9);

    // 0 has 6 bars, is contained within 8, contains 1 and is not equal to 9
    let zero = pattern.iter()
        .filter(|x| {
            !codec.has_symbol_seq(x) &&
            x.len() == 6 &&
                x.contains_all_bars_of(codec.encode(1).unwrap()) &&
                !x.eq(&nine)
        }).next().unwrap();

    codec.add_pair(zero.clone(), 0);

    // 6 has 6 bars, is not equal to 9 and not equal to 0
    let six = pattern.iter()
        .filter(|x| {
            !codec.has_symbol_seq(x) &&
            x.len() == 6 &&
                !x.eq(&zero) &&
                !x.eq(&nine)
        }).next().unwrap();

    codec.add_pair(six.clone(), 6);

    // 6 includes all bars which 5 has
    let five = pattern.iter()
        .filter(|x| {
            !codec.has_symbol_seq(x) &&
                six.contains_all_bars_of(x)
        }).next().unwrap();

    codec.add_pair(five.clone(), 5);

    // 2 is the last one
    let two = pattern.iter()
        .filter(|x| { !codec.has_symbol_seq(x) })
        .next().unwrap();

    codec.add_pair(two.clone(), 2);

    return digits_output.iter()
        .map(|d| *codec.decode(d).unwrap())
        .collect()
}

fn run_decoding(
    filename: &str,
    strategy: &dyn Fn(&Vec<String>, &Vec<String>) -> Vec<i32>,
    results_counter: &dyn Fn(&Vec<Vec<i32>>) -> u64
) -> u64 {
    let inputs: Vec<String> = read_lines_as_str_vector(filename);
    let decoded_lines = decode_lines(&inputs, strategy);
    return results_counter(&decoded_lines);
}

pub fn day_8() {
    let filename= "data/day-8-seven-segment-search/data-part-1.txt";
    let result = run_decoding(
        filename, &simple_strategy, &compute_result_part_1);

    println!("Day 8 Part 1 result: {res}", res=result);

    let result = run_decoding(
        filename, &full_search_strategy, &compute_result_part_2);

    println!("Day 8 Part 2 result: {res}", res=result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_digits_search() {
        let input = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe".to_string();
        assert_eq!(decode_line(&input, &simple_strategy), vec![8, -1, -1, 4])
    }

    #[test]
    fn test_run_decoding() {
        let filename= "data/day-8-seven-segment-search/test-data.txt";
        assert_eq!(run_decoding(filename, &simple_strategy, &compute_result_part_1), 26)
    }

    #[test]
    fn test_full_search() {
        let input = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe".to_string();
        assert_eq!(decode_line(&input, &full_search_strategy), vec![8, 3, 9, 4])
    }

    #[test]
    fn test_run_full_decoding() {
        let filename= "data/day-8-seven-segment-search/test-data.txt";
        assert_eq!(run_decoding(filename, &full_search_strategy, &compute_result_part_2), 61229)
    }

    #[test]
    fn test_charset_from_string() {
        assert_eq!(
            DigitSymbolSeq::from_string(&"abc".to_string()),
            DigitSymbolSeq::from_string(&"cab".to_string())
        );

        assert_ne!(
            DigitSymbolSeq::from_string(&"abcd".to_string()),
            DigitSymbolSeq::from_string(&"cab".to_string())
        );

        assert_ne!(
            DigitSymbolSeq::from_string(&"abc".to_string()),
            DigitSymbolSeq::from_string(&"cao".to_string())
        );
    }
}
