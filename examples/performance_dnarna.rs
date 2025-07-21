#[cfg(feature = "with_cpuprofiler")]
extern crate cpuprofiler;
extern crate endo_rs;

const ENDO_DNA: &str = include_str!("../data/endo.dna");

#[cfg(feature = "with_cpuprofiler")]
use cpuprofiler::PROFILER;
use endo_rs::DnaExecutor;

fn main() {
    let mut dna_executor = DnaExecutor::from(ENDO_DNA);
    let mut loops_count = 0;
    while !dna_executor.execute_loops(40000) {
        loops_count += 40000;
        println!("{}", loops_count);

        #[cfg(feature = "with_cpuprofiler")]
        {
            if loops_count >= 960_000 {
                println!("Starting collect...");
                PROFILER
                    .lock()
                    .unwrap()
                    .start("./performance_dnarna.profile")
                    .expect("Couldn't start");
            }

            if loops_count >= 1_000_000 {
                PROFILER.lock().unwrap().stop().expect("Couldn't stop");
                println!("Stopped collect.");
            }
        }
    }
}
