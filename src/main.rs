// 2023 day 3

#[derive(Debug)]
struct GearCandidate {
    line_index: usize,
    asterisk_index: usize,
    part_numbers: Vec<u32>,
}

impl GearCandidate {
    fn is_gear(&self) -> bool {
        if self.part_numbers.len() == 2 {
            true
        } else {
            false
        }
    }
    fn gear_ratio(&self) -> u32 {
        self.part_numbers[0] * self.part_numbers[1]
    }
}

fn main() {
    part1();
    part2();
}

fn part2() {
    let lines: Vec<&str> = include_str!("./input.txt").lines().collect();
    let mut lines_numbers: Vec<Vec<&str>> = vec![];
    let mut numbers: Vec<&str>;

    let mut lines_asterisks_positions: Vec<Vec<usize>> = vec![];

    for (_i, line) in lines.iter().enumerate() {

        // get all astersisk positions on line
        lines_asterisks_positions.push(find_all_occurrences_symbols(line, "*"));

        // get all numbers on line
        numbers = line
            .split(|c: char| !c.is_numeric())
            .filter(|&s| !s.is_empty())
            .collect();
        numbers.sort_unstable();
        numbers.dedup();
        lines_numbers.push(numbers.clone());
    }

    let mut gear_candidates: Vec<GearCandidate> = vec![];

    for (i, ast_positions) in lines_asterisks_positions.iter().enumerate() {

        for (j, ast_pos) in ast_positions.iter().enumerate() {
            let mut part_numbers: Vec<u32> = vec![];

            // check previous line
            if i > 0 {
                for num_str in lines_numbers[i - 1].iter() {
                    for num_pos in find_all_occurrences_numbers(lines[i - 1], num_str) {
                        let nr = num_range(num_str, num_pos, lines[i - 1]);
                        if ast_pos >= &nr.0 && ast_pos <= &nr.1 {
                            part_numbers.push(num_str.parse().unwrap());
                        }
                    }
                }
            }

            // check current line
            for num_str in lines_numbers[i].iter() {
                for num_pos in find_all_occurrences_numbers(lines[i], num_str) {
                    let nr = num_range(num_str, num_pos, lines[i]);
                    if ast_pos >= &nr.0 && ast_pos <= &nr.1 {
                        part_numbers.push(num_str.parse().unwrap());
                    }
                }
            }

            //check next line
            if i < (lines.len() - 1) {
                for num_str in lines_numbers[i + 1].iter() {
                    for num_pos in find_all_occurrences_numbers(lines[i + 1], num_str) {
                        let nr = num_range(num_str, num_pos, lines[i + 1]);
                        if ast_pos >= &nr.0 && ast_pos <= &nr.1 {
                            part_numbers.push(num_str.parse().unwrap());
                        }
                    }
                }
            }
            gear_candidates.push(GearCandidate {
                line_index: i,
                asterisk_index: j,
                part_numbers: part_numbers.clone(),
            });
        }
    }

    let mut sum: u32 = 0;
    for gear_candidate in gear_candidates.iter() {
        if gear_candidate.is_gear() {
            sum += gear_candidate.gear_ratio();
        }
    }
    println!("sum of gear ratios: {}", sum);
}

fn find_all_occurrences_symbols(main_str: &str, substring: &str) -> Vec<usize> {
    let mut positions = Vec::new();
    let mut start = 0;

    while let Some(pos) = main_str[start..].find(substring) {
        let absolute_pos = start + pos;

        positions.push(absolute_pos);
        start = absolute_pos + substring.len();
    }

    positions
}

fn num_range(num_str: &str, num_pos: usize, line_str: &str) -> (usize, usize) {
    let mut nr: (usize, usize) = (0, 0);
    if num_pos == 0 {
        nr = (0, num_str.len());
    }
    if num_pos > 0 {
        if num_pos < (line_str.len() - num_str.len()) {
            nr = (num_pos - 1, num_pos + num_str.len());
        } else {
            nr = (num_pos - 1, num_pos + num_str.len() - 1);
        }
    }

    nr
}

fn find_all_occurrences_numbers(main_str: &str, substring: &str) -> Vec<usize> {
    let mut positions = Vec::new();
    let mut start = 0;

    while let Some(pos) = main_str[start..].find(substring) {
        let absolute_pos = start + pos;

        // check if substring surrounded by non numeric chars and only push if true
        if absolute_pos == 0 {
            let slice =
                &main_str[(absolute_pos + substring.len())..(absolute_pos + substring.len() + 1)];
            let c = slice.chars().next().unwrap();
            if !c.is_numeric() {
                positions.push(absolute_pos);
            }
        }
        if absolute_pos > 0 && absolute_pos < (main_str.len() - substring.len()) {
            let slice = &main_str[(absolute_pos - 1)..absolute_pos];
            let c_before = slice.chars().next().unwrap();

            let slice =
                &main_str[(absolute_pos + substring.len())..(absolute_pos + substring.len() + 1)];
            let c_after = slice.chars().next().unwrap();
            if !c_before.is_numeric() && !c_after.is_numeric() {
                positions.push(absolute_pos);
            }
        }
        if absolute_pos == (main_str.len() - substring.len()) {
            let slice = &main_str[(absolute_pos - 1)..absolute_pos];
            let c_before = slice.chars().next().unwrap();
            if !c_before.is_numeric() {
                positions.push(absolute_pos);
            }
        }
        start = absolute_pos + substring.len();
    }

    positions
}

fn is_symbol_near(line: &str, symbols: Vec<&str>, start: usize, length: usize) -> bool {
    let mut flag = false;
    if start == 0 {
        let slice = &line[0..(length + 1)];
        for symbol in &symbols {
            if slice.contains(symbol) {
                flag = true;
            }
        }
    }
    if start > 0 && start <= (line.len() - length - 1) {
        let slice = &line[(start - 1)..(start + length + 1)];
        for symbol in &symbols {
            if slice.contains(symbol) {
                flag = true;
            }
        }
    }
    if start > (line.len() - length - 1) {
        let slice = &line[(start - 1)..];
        for symbol in &symbols {
            if slice.contains(symbol) {
                flag = true;
            }
        }
    }

    flag
}

#[allow(dead_code)]
fn part1() {
    let lines: Vec<&str> = include_str!("./input.txt").lines().collect();
    let mut lines_numbers: Vec<Vec<&str>> = vec![];
    let mut lines_symbols: Vec<Vec<&str>> = vec![];
    let mut numbers: Vec<&str>;
    let mut symbols: Vec<&str>;

    //for (i, line) in lines.iter().enumerate() {
    for line in lines.iter() {
        // get all numbers on line
        numbers = line
            .split(|c: char| !c.is_numeric())
            .filter(|&s| !s.is_empty())
            .collect();
        numbers.sort_unstable();
        numbers.dedup();
        lines_numbers.push(numbers.clone());

        // get all symbols on line
        symbols = line
            .split(|c: char| c.is_numeric() || c == '.' || c == '\n' || c == 'r')
            .filter(|&s| !s.is_empty())
            .collect();
        symbols.sort_unstable();
        symbols.dedup();
        lines_symbols.push(symbols.clone());
    }

    let mut sum_of_part_numbers: u32 = 0;
    for (i, line) in lines.iter().enumerate() {
        for n in lines_numbers[i].iter() {
            let mut part_number_flag = false;
            let length = n.len();
            let positions = find_all_occurrences_numbers(line, n);

            for position in positions {
                // check previous line
                if i != 0
                    && is_symbol_near(lines[i - 1], lines_symbols[i - 1].clone(), position, length)
                {
                    part_number_flag = true;
                }
                // check current line
                if is_symbol_near(lines[i], lines_symbols[i].clone(), position, length) {
                    part_number_flag = true;
                }

                // check next line
                if i != (lines.len() - 1)
                    && is_symbol_near(lines[i + 1], lines_symbols[i + 1].clone(), position, length)
                {
                    part_number_flag = true;
                }
                if part_number_flag {
                    sum_of_part_numbers += n.parse::<u32>().expect("cannot parse n to u32");
                }
            }
        }
    }
    println!("sum of part numbers: {}", sum_of_part_numbers);
}
