extern crate endo_rs;

const ENDO_DNA: &str = include_str!("../data/endo.dna");

use endo_rs::DnaExecutor;

fn main() {
    let dna = "IIPIFFCPICICIICPIICIPPPICIIC".to_string() + ENDO_DNA;

    let mut dna_executor = DnaExecutor::from(dna.as_str());
    dna_executor.execute();
}
