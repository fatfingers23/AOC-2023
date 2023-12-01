mod day_one {
    use std::collections::HashMap;
    use std::fs;

    fn calculate_string_cal_value(line: &str) -> i32 {
        let digits: String = line.chars().filter(|char| char.is_digit(10)).collect();
        let calibration_value: i32;
        if digits.len() == 1 {
            let double_up = format!("{}{}", digits, digits);
            calibration_value = double_up.parse::<i32>().unwrap();
        }
        else {
            let digits_split = digits.chars();
            let digits_split = digits_split.collect::<Vec<char>>();
            let string_cal_value = format!("{}{}", digits_split[0],  digits_split[digits_split.len() - 1]);
            calibration_value = string_cal_value.parse::<i32>().unwrap();
        }
        calibration_value
    }

    #[tokio::test]
    async fn sample_one() {
        let filename = "src/flat_files/test_data/one/sample_one.txt";
        let contents = fs::read_to_string(filename)
            .expect("Something went wrong reading the file");

        let mut calibration_total: i32 = 0;
        for line in contents.lines() {

            calibration_total += calculate_string_cal_value(line);
        }
        println!("Calibration Total: {}", calibration_total);
        assert_eq!(calibration_total, 142);
    }

    #[tokio::test]
    async fn one() {
        let filename = "src/flat_files/day_one_pt_one.txt";
        let contents = fs::read_to_string(filename)
            .expect("Something went wrong reading the file");

        let mut calibration_total: i32 = 0;
        for line in contents.lines() {
            calibration_total += calculate_string_cal_value(line);
        }
        println!("Calibration Total: {}", calibration_total);
        assert_eq!(calibration_total, 54_916);
    }


    fn get_numbers_in_words() -> HashMap<String, i32> {
        let mut numbers_in_words: HashMap<String, i32> = HashMap::new();

        numbers_in_words.insert("one".to_string(), 1);
        numbers_in_words.insert("two".to_string(), 2);
        numbers_in_words.insert("three".to_string(), 3);
        numbers_in_words.insert("four".to_string(), 4);
        numbers_in_words.insert("five".to_string(), 5);
        numbers_in_words.insert("six".to_string(), 6);
        numbers_in_words.insert("seven".to_string(), 7);
        numbers_in_words.insert("eight".to_string(), 8);
        numbers_in_words.insert("nine".to_string(), 9);

        numbers_in_words
    }


    /// The actual solution as a method to use on sample list as well
    fn pt_two_solution(file_loc: String) -> i32{
        let contents = fs::read_to_string(file_loc)
            .expect("Something went wrong reading the file");
        let mut calibration_total: i32 = 0;
        let numbers_in_words = get_numbers_in_words();

        for line in contents.lines() {
            let mut string_rankings: HashMap<i32, i32> = HashMap::new();

            //Gets the words that are numbers and their position
            for number in numbers_in_words.iter() {
                let word = number.0;
                if line.contains(word) {
                    let value_locations = line.match_indices(word).collect::<Vec<_>>();
                    for loc in value_locations {
                        //Add their position in the string to the hashmap as the key to get a "scoring" to know which came first
                        string_rankings.insert(loc.0 as i32, *number.1);
                    }
                }
            }
            let mut char_index = 0;
            for char in line.chars() {
                if char.is_digit(10) {
                    //Finds if the char is a digit under 10 and gets its position in the string for the "scoring"
                    string_rankings.insert(char_index, char.to_digit(10).unwrap() as i32);
                }
                char_index += 1;
            }

            //Sorts the hashmap by the key which is the position in the string to find which came first
            let mut sorted_cal: Vec<(&i32, &i32)> = string_rankings.iter().collect();
            sorted_cal.sort_by(|a, b| a.0.cmp(b.0));

            //Creates a string representation of the first and last values in the string to get the value
            let string_cal_value = format!("{}{}", sorted_cal.first().unwrap().1, sorted_cal.last().unwrap().1);

            let line_cal_value = string_cal_value.parse::<i32>().unwrap();
            calibration_total += line_cal_value;

        }

        calibration_total
    }

    #[tokio::test]
    async fn sample_two() {
        let filename = "src/flat_files/test_data/one/sample_two.txt";
        let pt_two_calibration_total = pt_two_solution(filename.to_string());
        println!("Calibration Total: {}", pt_two_calibration_total);
        assert_eq!(pt_two_calibration_total, 281);
    }

    #[tokio::test]
    async fn two() {
        let filename = "src/flat_files/day_one_pt_one.txt";
        let pt_two_calibration_total = pt_two_solution(filename.to_string());
        println!("Calibration Total: {}", pt_two_calibration_total);
        assert_eq!(pt_two_calibration_total, 54_728);
    }
}
