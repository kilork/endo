extern crate endo;

const ENDO_DNA: &'static str = include_str!("../data/endo.dna");

use endo::DnaExecutor;

#[test]
fn selfcheck_compare_to_sampled() {
    let dna = "IIPIFFCPICICIICPIICIPPPICIIC".to_string() + ENDO_DNA;
    let mut dna_executor = DnaExecutor::from(dna.as_str());
    let rna = dna_executor.execute();
    let result = rna
        .iter()
        .map(|x| format!("{:?}", x))
        .collect::<Vec<String>>()
        .join("\n");
    let expected = include_str!("../data/selfcheck.sample");
    assert_eq!(result.len(), expected.len());
    assert_eq!(result, expected);
}
