extern crate endo;

const ENDO_DNA: &'static str = include_str!("../data/endo.dna");

use endo::DnaExecutor;

fn main() {

    let dna = "IIPIFFCPICICIICPIICIPPPICIIC".to_string() + ENDO_DNA;

    let mut dna_executor = DnaExecutor::from(dna.as_str());
    dna_executor.execute();

}
