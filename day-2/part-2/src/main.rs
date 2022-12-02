use std::fs;
use std::io;
use std::path::Path;

#[derive(Debug)]
enum Selection {
    Rock,
    Paper,
    Scissors,
}

impl Selection {
    pub fn score(&self) -> i32 {
        match &self {
            Selection::Rock => 1,
            Selection::Paper => 2,
            Selection::Scissors => 3,
        }
    }
}

#[derive(Debug)]
enum GameResult {
    Win,
    Draw,
    Loss,
}

impl GameResult {
    pub fn score(&self) -> i32 {
        match &self {
            GameResult::Win => 6,
            GameResult::Draw => 3,
            GameResult::Loss => 0,
        }
    }
}

struct Game(Selection, GameResult);

fn main() {
    if let Ok(games) = get_input_games(&Path::new("input")) {
        print!("Total Score: {}", get_total_score(games));
    };
}

fn get_input_games(path: &Path) -> io::Result<Vec<Game>> {
    let input = fs::read_to_string(path)?;
    let lines = input.split('\n');
    Ok(lines.filter_map(|line| get_game_from_line(line)).collect())
}

fn get_game_from_line(line: &str) -> Option<Game> {
    if line.is_empty() {
        return None;
    };
    let first_letter = line.chars().nth(0).unwrap();
    let third_letter = line.chars().nth(2).unwrap();
    let opponent_selection = match first_letter {
        'A' => Selection::Rock,
        'B' => Selection::Paper,
        'C' => Selection::Scissors,
        _ => panic!("Unexpected character '{}' in input", first_letter),
    };
    let game_result = match third_letter {
        'X' => GameResult::Loss,
        'Y' => GameResult::Draw,
        'Z' => GameResult::Win,
        _ => panic!("Unexpected character '{}' in input", third_letter),
    };
    Some(Game(opponent_selection, game_result))
}

fn get_total_score(games: Vec<Game>) -> i32 {
    games.into_iter().map(|game| score_game(&game)).sum()
}

fn score_game(game: &Game) -> i32 {
    derive_my_selection(&game.0, &game.1).score() + &game.1.score()
}

fn derive_my_selection(opponent_selection: &Selection, result: &GameResult) -> Selection {
    match &opponent_selection {
        Selection::Rock => when_opponent_chooses_rock(&result),
        Selection::Paper => when_opponent_chooses_paper(&result),
        Selection::Scissors => when_opponent_chooses_scissors(&result),
    }
}

fn when_opponent_chooses_rock(result: &GameResult) -> Selection {
    match result {
        GameResult::Win => Selection::Paper,
        GameResult::Draw => Selection::Rock,
        GameResult::Loss => Selection::Scissors,
    }
}

fn when_opponent_chooses_paper(result: &GameResult) -> Selection {
    match result {
        GameResult::Win => Selection::Scissors,
        GameResult::Draw => Selection::Paper,
        GameResult::Loss => Selection::Rock,
    }
}

fn when_opponent_chooses_scissors(result: &GameResult) -> Selection {
    match result {
        GameResult::Win => Selection::Rock,
        GameResult::Draw => Selection::Scissors,
        GameResult::Loss => Selection::Paper,
    }
}
