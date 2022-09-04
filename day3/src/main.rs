use day3::{BitCounter,oxygen_gen_rating,co2_scrubber_rating};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut data: Vec<String> = vec![];

    let file = File::open("inputfile").expect("Failed to find file with input data");
    let reader = BufReader::new(file);
    for line in reader.lines().flatten() {
        data.push(line.parse().unwrap());
    }

    let mut bc = BitCounter::new(data[0].len());

    for line in &data {
        bc.update(line.as_ref());
    }

    println!("Power consumption: {}", bc.power_consumption());

    let tmpdata = data.iter().map(AsRef::as_ref).collect();
    println!("Oxygen generator rating is: {:?}", oxygen_gen_rating(&tmpdata).unwrap());
    println!("CO2 scrubber rating is: {:?}", co2_scrubber_rating(&tmpdata).unwrap());
}
