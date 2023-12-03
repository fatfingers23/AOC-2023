mod day_two {
    use std::collections::HashMap;
    use std::fs;


    const RED_CUBES: i32 = 12;
    const GREEN_CUBES: i32 = 13;
    const BLUE_CUBES: i32 = 14;


    #[derive(Debug)]
    struct  Game {
        pub game_id: i32,
        pub grabs: Vec<GameGrab>,
    }

    #[derive(Debug)]
    struct GameGrab {
        pub red_cubes: i32,
        pub green_cubes: i32,
        pub blue_cubes: i32,
    }



    fn parse_game(game_string: &str, game_id: i32) -> Game {
        let  get_number_and_color_regex = regex::Regex::new(r"(\d+)\s(\w+)").unwrap();
        let grabs_split = game_string.split(";").collect::<Vec<&str>>();

        let mut game = Game {
            game_id,
            grabs: vec![],
        };
        for grab in grabs_split {
            let colors_split = grab.split(",").collect::<Vec<&str>>();
            let mut red_total: i32 = 0;
            let mut green_total: i32 = 0;
            let mut blue_total: i32 = 0;
            for color in colors_split {
                let number_and_color_capture = get_number_and_color_regex.captures(color.trim());
                if let Some(captures) = number_and_color_capture {
                    let number = captures.get(1).unwrap().as_str();
                    let color = captures.get(2).unwrap().as_str();
                    match color {
                        "blue" => blue_total += number.parse::<i32>().unwrap(),
                        "green" => green_total += number.parse::<i32>().unwrap(),
                        "red" => red_total += number.parse::<i32>().unwrap(),
                        _ => {
                            println!("Unexpected color: {}", color);
                        }
                    }
                    // println!("Number: {}, Color: {}", number, color);
                }
            }
            let game_grab = GameGrab {
                red_cubes: red_total,
                green_cubes: green_total,
                blue_cubes: blue_total,
            };
            game.grabs.push(game_grab);
        }
        game
    }


    fn day_two_pt_one_solution(file_content: String) -> i32{
        let mut possible_games_ids_total = 0;
        let get_id_regex = regex::Regex::new(r"Game\s(\d+)").unwrap();
        for line in file_content.lines() {
            let game_id_split = line.split(":").collect::<Vec<&str>>();
            let game_id = get_id_regex.captures(game_id_split[0]).unwrap().get(1).unwrap().as_str().parse::<i32>().unwrap();
            let parsed_game = parse_game(game_id_split[1], game_id);
            let mut valid_game = true;

            for grab in parsed_game.grabs.iter() {

                if grab.red_cubes > RED_CUBES{
                    valid_game = false;
                }
                if grab.green_cubes > GREEN_CUBES{
                    valid_game = false;
                }
                if grab.blue_cubes > BLUE_CUBES{
                    valid_game = false;
                }
            }

            if valid_game {
                println!("Valid: Parsed Game: {:?}", parsed_game);
                possible_games_ids_total += game_id;
            }else{
                println!("Parsed Game: {:?}", parsed_game);
            }

        }
        possible_games_ids_total
    }



    #[tokio::test]
    async fn sample_one() {
        let filename = "src/flat_files/test_data/two/sample_one.txt";
        let contents = fs::read_to_string(filename)
            .expect("Something went wrong reading the file");
        let possible_games_ids_total = day_two_pt_one_solution(contents);
        assert_eq!(possible_games_ids_total, 8);
    }

    #[tokio::test]
    async fn two() {
        let filename = "src/flat_files/day_two_pt_one.txt";
        let contents = fs::read_to_string(filename)
            .expect("Something went wrong reading the file");
        let possible_games_ids_total = day_two_pt_one_solution(contents);
        println!("{}", possible_games_ids_total);
        assert_eq!(possible_games_ids_total, 2_476);
    }

}
