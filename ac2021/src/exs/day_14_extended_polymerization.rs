use std::collections::HashMap;
use super::utils::{read_lines_as_str_vector, frequencies, lines_into_chunks};

#[derive(Debug, Eq, PartialEq, Clone)]
struct Template {
    pairs_stats: HashMap<String, u64>,
    last_char: char
}

impl Template {
    fn decode(s: &String) -> Template {
        let l = s.len();
        let mut pairs = Vec::with_capacity(l * 2);
        for i in 0..(l - 1) {
            let pair = format!(
                "{}{}",
                s.chars().nth(i).unwrap(),
                s.chars().nth(i + 1).unwrap()
            );
            pairs.push(pair);
        }
        let pairs_stats = frequencies(&pairs[..]);
        let last_char = s.chars().last().unwrap();
        return Template { pairs_stats, last_char }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Instruction {
    rules: HashMap<String, String>
}

impl Instruction {
    fn new(encoded_pairs: &Vec<String>) -> Instruction {
        return Instruction {
            rules: encoded_pairs.iter()
                .map(|l| l.split(" -> ").collect::<Vec<&str>>())
                .map(|pair| (pair[0].to_string(), pair[1].to_string()))
                .collect::<HashMap<String, String>>()
        }
    }

    fn pair_to_new_pairs(&self, pair: &String) -> Vec<String> {
        let first = pair.chars().nth(0).unwrap();
        let last = pair.chars().nth(1).unwrap();
        let new_char = self.rules.get(pair).unwrap();
        return vec![
            format!("{}{}", first, new_char),
            format!("{}{}", new_char, last)
        ]
    }

    fn len(&self) -> usize {
        return self.rules.len()
    }

    fn apply_template_naive(&self, template: &String) -> String {
        let template_len = template.len();
        let mut new_template: Vec<String> = Vec::with_capacity(template_len);
        let mut pairs = Vec::with_capacity(template_len * 2);
        for i in 0..(template_len - 1) {
            let v = format!(
                "{}{}",
                template.chars().nth(i).unwrap(),
                template.chars().nth(i + 1).unwrap()
            );
            pairs.push(v);
        }
        for pair in pairs {
            let third_elem = self.rules.get(&pair).unwrap();
            let triple = format!(
                "{}{}",
                pair.chars().nth(0).unwrap(),
                third_elem,
            );
            new_template.push(triple)
        }
        new_template.push(format!("{}", template.chars().last().unwrap()));
        return new_template.join("")
    }

    fn apply_template_optimised(&self, template: &Template) -> Template {
        let mut new_template = HashMap::new();
        for (k, v) in &template.pairs_stats {
            let new_pairs = Instruction::pair_to_new_pairs(self,k);
            for new_pair in new_pairs {
                let cnt = new_template.entry(new_pair)
                    .or_insert(0);
                *cnt += v
            }
        }
        return Template {
            pairs_stats: new_template,
            last_char: template.last_char
        }
    }
}

fn aggregate_naive(template: &String) -> HashMap<char, u64> {
    return frequencies(&template.chars().collect::<Vec<_>>())
}

fn aggregate_from_pairs_stats(template: &Template) -> HashMap<char, u64> {
    let mut agg = HashMap::new();
    for (k, v) in &template.pairs_stats {
        let first = k.chars().nth(0).unwrap();
        let cnt = agg.entry(first).or_insert(0);
        *cnt += v;
    }
    let cnt = agg.entry(template.last_char).or_insert(0);
    *cnt += 1;
    return agg
}

fn compute_result(stats: & HashMap<char, u64>) -> u64 {
    let mut max: u64 = 0;
    let mut min: u64 = u64::MAX;
    for v in stats.values() {
        if *v > max { max = *v };
        if *v < min { min = *v }
    }
    return max - min
}

fn apply_steps_and_compute_p1_result(
    init_template: &String,
    instruction: &Instruction,
    steps_cnt: u32,
) -> u64 {
    let mut template: String = init_template.clone();
    for _ in 0..steps_cnt {
        template = instruction.apply_template_naive(&template);
    }
    let aggregated = aggregate_naive(&template);
    return compute_result(&aggregated)
}

fn apply_steps_and_compute_p2_result(
    init_template: &String,
    instruction: &Instruction,
    steps_cnt: u32,
) -> u64 {
    let mut template = Template::decode(init_template);
    for _ in 0..steps_cnt {
        template = instruction.apply_template_optimised(&template);
    }
    let aggregated = aggregate_from_pairs_stats(&template);
    return compute_result(&aggregated)
}

fn read_and_compute_p1_result(lines: &Vec<String>, n_steps: u32) -> u64 {
    let (init_template, instruction_data) =
        lines_into_chunks(lines);
    return apply_steps_and_compute_p1_result(
        &init_template.first().unwrap(),
        &Instruction::new(&instruction_data),
        n_steps
    )
}

fn read_and_compute_p2_result(lines: &Vec<String>, n_steps: u32) -> u64 {
    let (init_template, instruction_data) =
        lines_into_chunks(lines);
    return apply_steps_and_compute_p2_result(
        &init_template.first().unwrap(),
        &Instruction::new(&instruction_data),
        n_steps
    )
}

pub fn day_14() {
    let filename= "data/day-14-extended-polymerization/data-part-1.txt";
    let lines = read_lines_as_str_vector(filename);
    let res = read_and_compute_p1_result(&lines, 10);
    println!("Day 14 Part 1 result: {res}", res=res);
    let res = read_and_compute_p2_result(&lines, 40);
    println!("Day 14 Part 2 result: {res}", res=res);
}

#[cfg(test)]
mod tests {
    use crate::exs::utils::strs_to_strings;
    use super::*;

    #[test]
    fn test_apply_template() {
        let inputs = strs_to_strings(&vec![
            "CH -> B",
            "HH -> N",
            "CB -> H",
            "NH -> C",
            "HB -> C",
            "HC -> B",
            "HN -> C",
            "NN -> C",
            "BH -> H",
            "NC -> B",
            "NB -> B",
            "BN -> B",
            "BB -> N",
            "BC -> B",
            "CC -> N",
            "CN -> C"
        ]);
        let instruction = Instruction::new(&inputs);
        assert_eq!(instruction.len(), 16);

        assert_eq!(instruction.apply_template_naive(
            &"NNCB".to_string()), "NCNBCHB");
        assert_eq!(instruction.apply_template_naive(
            &"NCNBCHB".to_string()), "NBCCNBBBCBHCB");
        assert_eq!(instruction.apply_template_naive(
            &"NBCCNBBBCBHCB".to_string()), "NBBBCNCCNBBNBNBBCHBHHBCHB")

    }

    #[test]
    fn test_read_and_compute_result() {
        let inputs = strs_to_strings(&vec![
            "NNCB",
            "",
            "CH -> B",
            "HH -> N",
            "CB -> H",
            "NH -> C",
            "HB -> C",
            "HC -> B",
            "HN -> C",
            "NN -> C",
            "BH -> H",
            "NC -> B",
            "NB -> B",
            "BN -> B",
            "BB -> N",
            "BC -> B",
            "CC -> N",
            "CN -> C"
        ]);
        assert_eq!(read_and_compute_p1_result(&inputs, 10), 1588);
        assert_eq!(read_and_compute_p2_result(&inputs, 10), 1588);
    }

    fn test_template_from_string() {
        assert_eq!(
            Template::decode(&"NNCB".to_string()),
            Template {
                pairs_stats: hashmap![
                    "NN".to_string() => 1,
                    "NC".to_string() => 1,
                    "CB".to_string() => 1
                ],
                last_char: 'B'
            }
        );
        assert_eq!(
            Template::decode(&"NNCNNB".to_string()),
            Template {
                pairs_stats: hashmap![
                    "NN".to_string() => 2,
                    "NC".to_string() => 1,
                    "CB".to_string() => 1,
                    "CN".to_string() => 1,
                    "NB".to_string() => 1
                ],
                last_char: 'B'
            }
        )
    }

    #[test]
    fn test_apply_template_optimised() {
        let inputs = strs_to_strings(&vec![
            "CH -> B",
            "HH -> N",
            "CB -> H",
            "NH -> C",
            "HB -> C",
            "HC -> B",
            "HN -> C",
            "NN -> C",
            "BH -> H",
            "NC -> B",
            "NB -> B",
            "BN -> B",
            "BB -> N",
            "BC -> B",
            "CC -> N",
            "CN -> C"
        ]);
        let instruction = Instruction::new(&inputs);
        let template_0 = Template::decode(&"NNCB".to_string());
        assert_eq!(
            instruction.apply_template_optimised(&template_0),
            Template::decode(&"NCNBCHB".to_string())
        );
        assert_eq!(
            instruction.apply_template_optimised(&Template::decode(&"NCNBCHB".to_string())),
            Template::decode(&"NBCCNBBBCBHCB".to_string())
        );
    }

    #[test]
    fn test_aggregate_from_pairs_stats() {
        assert_eq!(
            aggregate_from_pairs_stats(&Template::decode(&"NBCCNBBBCBHCB".to_string())),
            hashmap!['N' => 2, 'C' => 4, 'B' => 6, 'H' => 1]
        )
    }
}
