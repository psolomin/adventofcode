use crate::exs::utils::read_lines_as_str_vector;

fn compute_frequencies<const N_BITS: usize>(
    diagnostic_report: &Vec<String>
) -> [[u32; 2]; N_BITS] {
    let mut frequencies = [[0u32; 2]; N_BITS];
    for observation in diagnostic_report {
        let mut chars = observation.chars();

        for idx in 0..N_BITS {
            if chars.next().unwrap() == '1' { frequencies[idx][0] += 1 }
            else { frequencies[idx][1] += 1 }
        }
    }
    return frequencies;
}

fn compute_most_common_and_least_common<const N_BITS: usize>(
    frequencies: [[u32; 2]; N_BITS]
) -> ([&'static str; N_BITS], [&'static str; N_BITS]) {
    let mut most_common = [""; N_BITS];
    let mut least_common = [""; N_BITS];

    for idx in 0..N_BITS {
        let bit_frequencies = frequencies[idx];
        if bit_frequencies[0] >= bit_frequencies[1] {
            most_common[idx] = "1";
            least_common[idx] = "0"
        }
        else {
            most_common[idx] = "0";
            least_common[idx] = "1"
        };
    }
    return (most_common, least_common);
}

fn str_bits_to_number<const N_BITS: usize>(bits_repr: &[&'static str; N_BITS]) -> isize {
    return isize::from_str_radix(&bits_repr.join(""), 2).unwrap();
}

fn str_to_number(input: &String) -> isize {
    return isize::from_str_radix(input,  2).unwrap();
}

fn compute_power_rate<const N_BITS: usize>(diagnostic_report: &Vec<String>) -> u32 {
    let frequencies =
        compute_frequencies::<N_BITS>(diagnostic_report);

    let (most_common, least_common) =
        compute_most_common_and_least_common::<N_BITS>(frequencies);

    let gamma = str_bits_to_number::<N_BITS>(&most_common);
    let epsilon = str_bits_to_number::<N_BITS>(&least_common);
    return gamma as u32 * epsilon as u32;
}

fn extract_with_crazy_rule<const N_BITS: usize>(
    diagnostic_report: &Vec<String>,
    pick_most_common: bool
) -> u32 {
    let mut slice = diagnostic_report;
    let mut slice_data: Vec<String>;

    for idx in 0..N_BITS {
        if slice.len() == 1 { return str_to_number(slice.first().unwrap()) as u32 }

        let frequencies =
            compute_frequencies::<N_BITS>(slice);

        let (most_common, least_common) =
            compute_most_common_and_least_common::<N_BITS>(frequencies);

        let current_to_select: char;
        if pick_most_common {
            current_to_select = most_common[idx].chars().nth(0).unwrap();
        } else {
            current_to_select = least_common[idx].chars().nth(0).unwrap();
        }
        let current_frequencies = frequencies[idx];
        let current_to_select_frequency =
            if current_to_select == '1' { current_frequencies[0] }
            else { current_frequencies[1] };

        slice_data = slice.iter()
            .filter(|r| r.chars().nth(idx).unwrap() == current_to_select)
            .map(|r| r.clone())
            .take(current_to_select_frequency as usize)
            .collect();

        slice = &slice_data;
    }
    return str_to_number(slice.first().unwrap()) as u32;
}

fn compute_oxygen_generator_rating<const N_BITS: usize>(diagnostic_report: &Vec<String>) -> u32 {
    return extract_with_crazy_rule::<N_BITS>(diagnostic_report, true)
}

fn compute_co2_scrubber_rating<const N_BITS: usize>(diagnostic_report: &Vec<String>) -> u32 {
    return extract_with_crazy_rule::<N_BITS>(diagnostic_report, false)
}

pub fn day_3() {
    const BITS_CNT_ACTUAL: usize = 12usize;
    let filename = "data/day-3-binary-diagnostic/data-part-1.txt";
    let observations = read_lines_as_str_vector(filename);
    let result = compute_power_rate::<BITS_CNT_ACTUAL>(&observations);
    println!("Day 3 Part 1 result: {res}", res=result);
    let ox = compute_oxygen_generator_rating::<BITS_CNT_ACTUAL>(&observations);
    let co2 = compute_co2_scrubber_rating::<BITS_CNT_ACTUAL>(&observations);
    println!("Day 3 Part 2 result: {res}", res=ox * co2);
}

#[cfg(test)]
mod tests {
    use crate::exs::utils::strs_to_strings;
    use super::*;

    #[test]
    fn test_calc_final_pos_and_depth() {
        let inputs = strs_to_strings(&vec![
            "00100",
            "11110",
            "10110",
            "10111",
            "10101",
            "01111",
            "00111",
            "11100",
            "10000",
            "11001",
            "00010",
            "01010"
        ]);
        const BITS_CNT: usize = 5usize;
        assert_eq!(compute_power_rate::<BITS_CNT>(&inputs), 198);
    }

    #[test]
    fn test_calc_final_pos_and_depth_equal_frequency() {
        let inputs = strs_to_strings(&vec![
            "00100",
            "11110"
        ]);
        const BITS_CNT: usize = 5usize;
        assert_eq!(compute_power_rate::<BITS_CNT>(&inputs), 30);
    }

    #[test]
    fn test_compute_oxygen_generator_rating() {
        let inputs = strs_to_strings(&vec![
            "00100",
            "11110",
            "10110",
            "10111",
            "10101",
            "01111",
            "00111",
            "11100",
            "10000",
            "11001",
            "00010",
            "01010"
        ]);
        const BITS_CNT: usize = 5usize;
        assert_eq!(compute_oxygen_generator_rating::<BITS_CNT>(&inputs), 23);
    }

    #[test]
    fn test_compute_oxygen_generator_rating_early_stop() {
        let inputs = strs_to_strings(&vec![
            "00100",
            "11110",
            "10110"
        ]);
        const BITS_CNT: usize = 5usize;
        assert_eq!(compute_oxygen_generator_rating::<BITS_CNT>(&inputs), 30);
    }

    #[test]
    fn test_compute_co2_scrubber_rating() {
        let inputs = strs_to_strings(&vec![
            "00100",
            "11110",
            "10110",
            "10111",
            "10101",
            "01111",
            "00111",
            "11100",
            "10000",
            "11001",
            "00010",
            "01010"
        ]);
        const BITS_CNT: usize = 5usize;
        assert_eq!(compute_co2_scrubber_rating::<BITS_CNT>(&inputs), 10);
    }
}
