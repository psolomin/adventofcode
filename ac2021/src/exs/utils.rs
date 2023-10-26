#![macro_use]

use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::{BufRead, BufReader, Lines};
use std::iter::Map;
use std::str::FromStr;


#[macro_export]
macro_rules! modulus {
    ($a:expr, $b:expr) => { (($a % $b) + $b) % $b }
}

#[macro_export]
macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

#[macro_export]
macro_rules! hashset {
    ($( $val: expr ),*) => {{
         let mut s = ::std::collections::HashSet::new();
         $( s.insert($val); )*
         s
    }}
}

pub fn read_lines_as_vector<T>(file_path: &str) -> Vec<T>
    where T: FromStr, <T as FromStr>::Err: std::fmt::Debug
{
    return lines_reader(file_path)
        .map(|line| line.parse::<T>().unwrap())
        .collect();
}

pub fn read_lines_as_str_vector(file_path: &str) -> Vec<String> {
    return lines_reader(file_path).map(|r| r.to_string()).collect();
}

pub fn lines_reader(file_path: &str) -> Map<Lines<BufReader<File>>, fn(std::io::Result<String>) -> String> {
    let file = File::open(file_path).expect("no such file");
    let buf = BufReader::new(file);
    return buf.lines()
        .map(|line| line.expect("Could not parse line"))
}

pub fn open_range_vec(start: u32, end: u32) -> Vec<u32> {
    if end >= start { (start..=end).collect::<Vec<u32>>() }
    else { (end..=start).rev().collect::<Vec<u32>>() }
}

pub fn strs_to_strings(strs: &[&str]) -> Vec<String> {
    return strs.iter().map(|s| s.to_string()).collect()
}

pub fn lines_into_chunks(lines: &Vec<String>) -> (Vec<String>, Vec<String>) {
    let lines_of_dots: Vec<String> = lines.iter()
        .take_while(|l| l != &"").map(|l| l.clone()).collect();
    let folds_start_after = lines_of_dots.len();
    let folds_lines = lines[(folds_start_after + 1)..].to_vec();
    return (lines_of_dots, folds_lines)
}

pub fn frequencies<T: Eq + Hash + Clone>(input: &[T]) -> HashMap<T, u64> {
    let mut output = HashMap::new();
    for item in input {
        let cnt = output.entry(item.clone()).or_insert(0);
        *cnt += 1;
    }
    return output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modulus() {
        assert_eq!(modulus!(7, 3), 1);
        assert_eq!(modulus!(7u8, 3u8), 1u8);
        assert_eq!(modulus!(7u64, 3u64), 1u64);
        assert_eq!(modulus!(-21, 4), 3);
        assert_eq!(modulus!(6, 3), 0);
    }

    #[test]
    fn test_frequencies() {
        assert_eq!(frequencies(&vec![7, 3]), hashmap![7 => 1, 3 => 1]);
        assert_eq!(frequencies(&vec![7, 3, 3]), hashmap![7 => 1, 3 => 2]);
    }
}
