extern crate endo;

const ENDO_DNA: &'static str = include_str!("../data/endo.dna");

use endo::DnaExecutor;

fn main() {
    let mut dna_executor = DnaExecutor::from(ENDO_DNA);
    let mut loops_count = 0;
    while !dna_executor.execute_loops(10000) {
        loops_count += 10000;
        println!("{}", loops_count);
    }
}
