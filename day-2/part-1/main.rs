use std::env;
use std::fs;
use std::io;
use std::path::Path;

#[derive(Debug)]
enum Selection {
    Rock,
    Paper,
    Scissors,
}

enum GameResult {
    Win,
    Draw,
    Loss,
}

#[derive(Debug)]
struct Entry(Selection, Selection);

fn main() {
    if let Ok(entries) = get_input_entries(&Path::new("input")) {
        print!("Total Score: {}", get_total_score(entries));
    };
}

fn get_input_entries(path: &Path) -> io::Result<Vec<Entry>> {
    let input = fs::read_to_string(path)?;
    let lines = input.split('\n');
    Ok(lines.filter_map(|line| get_entry_from_line(line)).collect())
}

fn get_total_score(entries: Vec<Entry>) -> i32 {
    entries
        .into_iter()
        .map(|entry| entry_into_score(entry))
        .sum()
}

fn get_entry_from_line(line: &str) -> Option<Entry> {
    if line.is_empty() {
        return None;
    };
    let first_letter = line.chars().nth(0).unwrap();
    let third_letter = line.chars().nth(2).unwrap();
    let my_selection = match first_letter {
        'A' => Selection::Rock,
        'B' => Selection::Paper,
        'C' => Selection::Scissors,
        _ => panic!("Unexpected character '{}' in input", first_letter),
    };
    let opponent_selection = match third_letter {
        'X' => Selection::Rock,
        'Y' => Selection::Paper,
        'Z' => Selection::Scissors,
        _ => panic!("Unexpected character '{}' in input", third_letter),
    };
    Some(Entry(opponent_selection, my_selection))
}

fn entry_into_score(entry: Entry) -> i32 {
    score_selection(&entry.0) + score_result(compare_selections_in_entry(&entry))
}

fn score_selection(selection: &Selection) -> i32 {
    match selection {
        Selection::Rock => 1,
        Selection::Paper => 2,
        Selection::Scissors => 3,
    }
}

fn compare_selections_in_entry(entry: &Entry) -> GameResult {
    match (&entry.0, &entry.1) {
        (Selection::Rock, Selection::Rock) => GameResult::Draw,
        (Selection::Rock, Selection::Paper) => GameResult::Loss,
        (Selection::Rock, Selection::Scissors) => GameResult::Win,
        (Selection::Paper, Selection::Paper) => GameResult::Draw,
        (Selection::Paper, Selection::Scissors) => GameResult::Loss,
        (Selection::Paper, Selection::Rock) => GameResult::Win,
        (Selection::Scissors, Selection::Scissors) => GameResult::Draw,
        (Selection::Scissors, Selection::Rock) => GameResult::Loss,
        (Selection::Scissors, Selection::Paper) => GameResult::Win,
    }
}

fn score_result(result: GameResult) -> i32 {
    match result {
        GameResult::Win => 6,
        GameResult::Draw => 3,
        GameResult::Loss => 0,
    }
}
