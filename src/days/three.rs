mod day_two {
    use std::fs;

    #[derive(Debug, Clone)]
    struct PartNumberDetail {
        pub part_number: i32,
        pub line_number: i32,
        pub start_loc: i32,
        pub end_loc: i32,
    }

    #[derive(Debug)]
    struct SymbolDetail {
        pub symbol: String,
        pub line_number: i32,
        pub loc: i32,
    }

    struct Schematic {
        pub part_number_details: Vec<PartNumberDetail>,
        pub symbol_details: Vec<SymbolDetail>,
    }

    fn schematic_parser(schematic: String) -> Schematic {
        let mut part_number_details: Vec<PartNumberDetail> = Vec::new();
        let mut symbol_details: Vec<SymbolDetail> = Vec::new();
        let mut line_number = 1;
        for line in schematic.lines() {
            let mut number_as_string: String = String::new();
            let mut number_start_loc = 0;
            let mut char_index = 1;
            for char in line.chars() {
                if char.is_digit(10) {
                    if number_as_string.len() == 0 {
                        number_start_loc = char_index;
                    }
                    number_as_string.push(char);
                } else {
                    if char != '.' {
                        let symbol_detail = SymbolDetail {
                            symbol: char.to_string(),
                            line_number,
                            loc: char_index,
                        };
                        symbol_details.push(symbol_detail);
                        // print!("{:?}\n", symbol_detail);
                    }

                    if number_as_string.len() > 0 {
                        let part_number = number_as_string.parse::<i32>().unwrap();
                        let part_number_detail = PartNumberDetail {
                            part_number,
                            line_number,
                            start_loc: number_start_loc,
                            end_loc: char_index,
                        };
                        // print!("{:?}\n", part_number_detail);
                        part_number_details.push(part_number_detail);
                    }
                    number_as_string = String::new();
                }

                if char_index == line.len() as i32 {
                    if number_as_string.len() > 0 {
                        let part_number = number_as_string.parse::<i32>().unwrap();
                        let part_number_detail = PartNumberDetail {
                            part_number,
                            line_number,
                            start_loc: number_start_loc,
                            end_loc: char_index,
                        };
                        part_number_details.push(part_number_detail);
                    }
                }
                char_index += 1;
            }
            line_number += 1;
        }
        Schematic {
            part_number_details,
            symbol_details,
        }
    }

    fn get_line_valid_positions(start_loc: i32, end_loc: i32) -> Vec<i32> {
        let mut valid_positions: Vec<i32> = Vec::new();
        for valid_position in start_loc - 1..end_loc + 1 {
            valid_positions.push(valid_position);
        }
        valid_positions
    }

    fn check_if_adjacent_symbol(
        symbol_details: Vec<&SymbolDetail>,
        valid_locations: Vec<i32>,
    ) -> bool {
        let mut is_there_an_adjacent_symbol = false;
        for symbol_detail in symbol_details {
            if valid_locations.contains(&symbol_detail.loc) {
                is_there_an_adjacent_symbol = true;
            }
        }
        is_there_an_adjacent_symbol
    }

    fn same_line_check(
        part_number_detail: PartNumberDetail,
        symbol_details: Vec<&SymbolDetail>,
    ) -> bool {
        let mut is_there_an_adjacent_symbol = false;

        for symbol_on_same_line in symbol_details {
            if symbol_on_same_line.loc == part_number_detail.start_loc - 1 {
                is_there_an_adjacent_symbol = true;
            }
            if symbol_on_same_line.loc + 1 == part_number_detail.end_loc + 1 {
                is_there_an_adjacent_symbol = true;
            }
        }
        is_there_an_adjacent_symbol
    }

    fn day_two_pt_one_solution(file_content: String) -> i32 {
        let mut schematic_part_number_sum = 0;
        let schematic = schematic_parser(file_content);
        for part_number_detail in schematic.part_number_details {
            let mut is_there_an_adjacent_symbol = false;
            let valid_locations =
                get_line_valid_positions(part_number_detail.start_loc, part_number_detail.end_loc);

            let symbols_before = schematic
                .symbol_details
                .iter()
                .filter(|symbol_detail| {
                    symbol_detail.line_number == part_number_detail.line_number - 1
                })
                .collect::<Vec<&SymbolDetail>>();
            let symbols_after = schematic
                .symbol_details
                .iter()
                .filter(|symbol_detail| {
                    symbol_detail.line_number == part_number_detail.line_number + 1
                })
                .collect::<Vec<&SymbolDetail>>();

            let symbols_on_same_line = schematic
                .symbol_details
                .iter()
                .filter(|symbol_detail| symbol_detail.line_number == part_number_detail.line_number)
                .collect::<Vec<&SymbolDetail>>();

            let line_before_check =
                check_if_adjacent_symbol(symbols_before, valid_locations.clone());
            let line_after_check = check_if_adjacent_symbol(symbols_after, valid_locations.clone());
            let same_line_check = same_line_check(part_number_detail.clone(), symbols_on_same_line);

            if part_number_detail.part_number.is_negative() {
                println!("NEGATIVE {:?}", part_number_detail);
                continue;
            }

            if line_before_check || line_after_check || same_line_check {
                is_there_an_adjacent_symbol = true;
            }

            if is_there_an_adjacent_symbol {
                schematic_part_number_sum += part_number_detail.part_number;
            } else {
                println!("{:?}", part_number_detail);
            }
        }
        schematic_part_number_sum
    }

    fn day_two_pt_two_solution(file_content: String) -> i32 {
        let mut game_lowest_pow_sum = 0;

        game_lowest_pow_sum
    }

    #[tokio::test]
    async fn sample_one() {
        let filename = "src/flat_files/test_data/three/sample_one.txt";
        let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
        let valid_part_numbers_sum = day_two_pt_one_solution(contents);
        assert_eq!(valid_part_numbers_sum, 4361);
    }

    #[tokio::test]
    async fn one() {
        let filename = "src/flat_files/day_three.txt";
        let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
        let valid_part_numbers_sum = day_two_pt_one_solution(contents);
        assert_eq!(valid_part_numbers_sum, 530_495);
    }
    //
    // #[tokio::test]
    // async fn sample_two() {
    //     let filename = "src/flat_files/test_data/two/sample_two.txt";
    //     let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    //     let possible_games_ids_total = day_two_pt_two_solution(contents);
    //     assert_eq!(possible_games_ids_total, 2_286);
    // }
    //
    // #[tokio::test]
    // async fn two() {
    //     let filename = "src/flat_files/day_two.txt";
    //     let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    //     let possible_games_ids_total = day_two_pt_two_solution(contents);
    //     assert_eq!(possible_games_ids_total, 54_911);
    // }
}
