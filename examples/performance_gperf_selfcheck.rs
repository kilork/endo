extern crate cpuprofiler;
extern crate endo;

const ENDO_DNA: &'static str = include_str!("../data/endo.dna");

use cpuprofiler::PROFILER;
use endo::DnaExecutor;

fn main() {
    let dna = "IIPIFFCPICICIICPIICIPPPICIIC".to_string() + ENDO_DNA;
    let mut dna_executor = DnaExecutor::from(dna.as_str());

    // Unlock the mutex and start the profiler
    PROFILER
        .lock()
        .unwrap()
        .start("./performance_selfcheck.profile")
        .expect("Couldn't start");
    dna_executor.execute();

    // Unwrap the mutex and stop the profiler
    PROFILER.lock().unwrap().stop().expect("Couldn't stop");
}
