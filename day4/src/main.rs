use day4::Game;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let mut data: Vec<String> = vec![];

    let file = File::open("inputfile").expect("Failed to find file with input data");
    let reader = BufReader::new(file);
    for line in reader.lines().flatten() {
        data.push(line.parse().unwrap());
    }

    let mut it = data.into_iter();

    let draws = it.next().unwrap();
    let mut game: Game = FromStr::from_str(draws.as_ref())?;

    loop {
        if it.next().is_none() {
            break;
        }

        let board: String = it.by_ref().take(5).map(|mut s| { s.push('\n'); s} ).collect();
        game.add_player(FromStr::from_str(board.as_ref())?);
    }

    let (winning_draw, winner) = game.get_next_winner().unwrap().unwrap();
    println!("Winning draw: {winning_draw}");
    println!("Sum of first winner's unmarked numbers: {}", winner.sum_of_unmarked());

    while let Some((next_winning_draw, next_winner)) = game.get_next_winner().unwrap() {
        println!("Next winning draw: {next_winning_draw}");
        println!("Sum of next winner's unmarked numbers: {}", next_winner.sum_of_unmarked());
    }

    Ok(())
}
