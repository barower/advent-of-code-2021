use day13::FoldPaper;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("inputfile").expect("Failed to find file with input data");
    let reader = BufReader::new(file);
    let points: Vec<(usize, usize)> = reader
        .lines()
        .flatten()
        .map(|line| {
            let mut sp = line.split(',');
            (
                sp.next().unwrap().parse::<usize>().unwrap(),
                sp.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .collect();

    let mut paper = FoldPaper::new(points);

    paper.fold_along_x(655)?;

    println!("Result of part 1 is {}", paper.count_dots());

    paper.fold_along_y(447)?;
    paper.fold_along_x(327)?;
    paper.fold_along_y(223)?;
    paper.fold_along_x(163)?;
    paper.fold_along_y(111)?;
    paper.fold_along_x(81)?;
    paper.fold_along_y(55)?;
    paper.fold_along_x(40)?;
    paper.fold_along_y(27)?;
    paper.fold_along_y(13)?;
    paper.fold_along_y(6)?;

    println!("Result of part 2 is:\n{:?}", paper);

    Ok(())
}
