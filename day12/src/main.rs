use day12::{CaveGraph, PathsVariant};
use std::fs::File;
use std::io::{BufRead, BufReader};
use tikv_jemallocator::Jemalloc;

#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
        let mut graph = CaveGraph::default();

        let file = File::open("inputfile").expect("Failed to find file with input data");
        let reader = BufReader::new(file);
        for line in reader.lines().flatten() {
            graph.add_entry(line.as_ref())?;
        }

        println!("Result of part 1 is {}", graph.paths(PathsVariant::AllSmallCavesOnce)?.len());
        println!("Result of part 2 is {}", graph.paths(PathsVariant::OneSmallCaveTwice)?.len());

        Ok(())
}
