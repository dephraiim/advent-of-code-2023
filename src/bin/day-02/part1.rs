use std::collections::HashSet;
use std::fs;
use std::io;

pub fn run() -> io::Result<usize> {
    let input: String = fs::read_to_string("src/bin/day-02/input-1.txt")?;
    let lines: Vec<String> = input.split("\n").map(|s: &str| s.to_string()).collect();

    let mut answer_vec: Vec<usize> = vec![];

    let games: Vec<_> = lines
        .iter()
        .map(|g: &String| g.as_str().split(":").collect::<Vec<_>>()[1].trim())
        .collect();

    for (i, game) in games.iter().enumerate() {
        // split into different games
        let game_number = i + 1;
        let sub_games: Vec<_> = game.split(";").collect();

        for sg in sub_games {
            let sub_game: &str = sg.trim();
            let sub_sub_games: Vec<_> = sub_game.split(",").collect();

            for ssg in sub_sub_games {
                let game_played: Vec<&str> = ssg.trim().split_whitespace().collect::<Vec<_>>();
                let number = game_played[0]
                    .parse::<i32>()
                    .expect("Failed to parse the number");
                let name = game_played[1];

                if (name == "red") & (number > 12) {
                    answer_vec.push(game_number)
                }

                if (name == "green") & (number > 13) {
                    answer_vec.push(game_number)
                }

                if (name == "blue") & (number > 14) {
                    answer_vec.push(game_number)
                }
            }
        }
    }

    let v: HashSet<_> = answer_vec.iter().cloned().collect();
    let vec: Vec<_> = (1..=games.len()).collect();

    let result: Vec<_> = vec.into_iter().filter(|x| !v.contains(x)).collect();

    let mut answer = 0;

    for i in result {
        answer += i;
    }

    Ok(answer)
}
