// 2023 day 3

fn main() {
    part1();
    //part2();
}

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
            let positions = find_all_occurrences(line, n);

            for position in positions {
                // check previous line
                if i != 0 {
                    if is_symbol_near(lines[i - 1], lines_symbols[i - 1].clone(), position, length) {
                        part_number_flag = true;
                    }
                }
                // check current line
                if is_symbol_near(lines[i], lines_symbols[i].clone(), position, length) {
                    part_number_flag = true;
                }

                // check next line
                if i != (lines.len() - 1) {
                    if is_symbol_near(lines[i + 1], lines_symbols[i + 1].clone(), position, length) {
                        part_number_flag = true;
                    }
                }
                if part_number_flag {
                    sum_of_part_numbers += n.parse::<u32>().expect("cannot parse n to u32");
                }
            }
        }
    }
    println!("sum of part numbers: {}", sum_of_part_numbers);
}

fn find_all_occurrences(main_str: &str, substring: &str) -> Vec<usize> {
    let mut positions = Vec::new();
    let mut start = 0;

    while let Some(pos) = main_str[start..].find(substring) {
        let absolute_pos = start + pos;

        // check if substring surrounded by non numeric chars and only push if true
        if absolute_pos == 0 {
            let slice = &main_str[(absolute_pos+substring.len())..(absolute_pos+substring.len()+1)];
            let c = slice.chars().next().unwrap();
            if !c.is_numeric() {
                positions.push(absolute_pos);
            }
        }
        if absolute_pos > 0 && absolute_pos < (main_str.len() - substring.len()) {
            let slice = &main_str[(absolute_pos-1)..absolute_pos];
            let c_before = slice.chars().next().unwrap();

            let slice = &main_str[(absolute_pos+substring.len())..(absolute_pos+substring.len()+1)];
            let c_after = slice.chars().next().unwrap();
            if !c_before.is_numeric() && !c_after.is_numeric() {
                positions.push(absolute_pos);
            }
        }
        if absolute_pos == (main_str.len() - substring.len()) {
            let slice = &main_str[(absolute_pos-1)..absolute_pos];
            let c_before = slice.chars().next().unwrap();
            if !c_before.is_numeric() {
                positions.push(absolute_pos);
            }
        }
        //positions.push(absolute_pos);
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

// fn part2() {
// }
