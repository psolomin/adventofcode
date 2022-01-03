use super::utils::read_lines_as_str_vector;

fn analyse_line(line: &String) -> (Option<String>, Option<Vec<&str>>) {
    let mut state = Vec::new();
    let symbols = line.split("").filter(|x|x != &"");
    for symbol in symbols {
        match symbol {
            "(" => { state.push(")") }
            "{" => { state.push("}") }
            "[" => { state.push("]") }
            "<" => { state.push(">") }
            x=> {
                let expected_x = state.pop();
                match expected_x {
                    Some(expected_symbol) => {
                        if x == expected_symbol { continue }
                        else { return (Some(x.to_string()), None) }
                    }
                    None => { }
                }
            }
        }
    }

    return (None, Some(state))
}

fn compute_score(erroneous_symbols: &Vec<String>) -> u32 {
    let mut score = 0;
    for e in erroneous_symbols {
        score = score + match e.as_ref() {
            ")" => { 3 }
            "]" => { 57 }
            "}" => { 1197 }
            ">" => { 25137 }
            _ => { 0 }
        }
    }
    return score
}

fn compute_auto_complete_scores(missing_symbols_sets: &Vec<Vec<&str>>) -> Vec<u64> {
    let mut scores = Vec::new();
    for missing_symbol_set in missing_symbols_sets {
        let mut score = 0u64;
        for symbol in missing_symbol_set.iter().rev() {
            score = score * 5;
            score = score + match symbol.as_ref() {
                ")" => { 1 }
                "]" => { 2 }
                "}" => { 3 }
                ">" => { 4 }
                _ => { 0 }
            }
        }
        scores.push(score);
    }
    return scores
}

fn compute_auto_complete_score_median(scores: &Vec<u64>) -> u64 {
    let mut scores_to_sort = scores.clone();
    scores_to_sort.sort_by_key(|s| s.clone());
    return scores_to_sort[(scores_to_sort.len() / 2)]
}

fn syntax_error_score(lines: &Vec<String>) -> u32 {
    let mut first_erroneous_symbols = Vec::new();
    for line in lines {
        let (maybe_first_erroneous_symbol, _) = analyse_line(line);
        match maybe_first_erroneous_symbol {
            Some(err_symbol) => { first_erroneous_symbols.push(err_symbol) }
            None => {}
        }
    }
    return compute_score(&first_erroneous_symbols)
}

fn auto_complete_score(lines: &Vec<String>) -> u64 {
    let mut missing_symbols = Vec::new();
    for line in lines {
        let (_, symbols) = analyse_line(line);
        match symbols {
            Some(line_missing_symbols) => {
                missing_symbols.push(line_missing_symbols)
            }
            None => {}
        }
    }
    let scores = compute_auto_complete_scores(&missing_symbols);
    return compute_auto_complete_score_median(&scores);
}

pub fn day_10() {
    let filename= "data/day-10-syntax-scoring/data-part-1.txt";
    let lines = read_lines_as_str_vector(filename);
    let result = syntax_error_score(&lines);
    println!("Day 10 Part 1 result: {res}", res=result);
    let result2 = auto_complete_score(&lines);
    println!("Day 10 Part 2 result: {res}", res=result2);
}

#[cfg(test)]
mod tests {
    use crate::exs::utils::strs_to_strings;
    use super::*;

    #[test]
    fn test_syntax_error_score() {
        let input = strs_to_strings(&vec![
            "[({(<(())[]>[[{[]{<()<>>",
            "[(()[<>])]({[<{<<[]>>(",
            "{([(<{}[<>[]}>{[]{[(<()>",
            "(((({<>}<{<{<>}{[]{[]{}",
            "[[<[([]))<([[{}[[()]]]",
            "[{[{({}]{}}([{[{{{}}([]",
            "{<[[]]>}<{[{[{[]{()[[[]",
            "[<(<(<(<{}))><([]([]()",
            "<{([([[(<>()){}]>(<<{{",
            "<{([{{}}[<[[[<>{}]]]>[]]"
        ]);

        assert_eq!(syntax_error_score(&input), 26397)
    }

    #[test]
    fn test_auto_complete_score() {
        let input = strs_to_strings(&vec![
            "[({(<(())[]>[[{[]{<()<>>",
            "[(()[<>])]({[<{<<[]>>(",
            "{([(<{}[<>[]}>{[]{[(<()>",
            "(((({<>}<{<{<>}{[]{[]{}",
            "[[<[([]))<([[{}[[()]]]",
            "[{[{({}]{}}([{[{{{}}([]",
            "{<[[]]>}<{[{[{[]{()[[[]",
            "[<(<(<(<{}))><([]([]()",
            "<{([([[(<>()){}]>(<<{{",
            "<{([{{}}[<[[[<>{}]]]>[]]"
        ]);

        assert_eq!(auto_complete_score(&input), 288957)
    }
}
