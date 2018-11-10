extern crate log;
extern crate simplelog;

extern crate endo;

use log::info;
use simplelog::{CombinedLogger, Config, LevelFilter, TermLogger, WriteLogger};

use std::env::args;
use std::fs::File;

const ENDO_DNA: &'static str = include_str!("../data/endo.dna");

use endo::DnaExecutor;
use endo::{RnaRenderer, RNA};

fn main() {
    let _ = CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Info, Config::default()).unwrap(),
        WriteLogger::new(
            LevelFilter::Debug,
            Config::default(),
            File::create("dnarna.log").unwrap(),
        ),
    ]);
    // IPIFFCPICFPPICIICCIICIPPPFIIC - from first screen
    // IIPIFFCPICICIICPIICIPPPICIIC - self check
    let prefix = if let Some(prefix) = args().nth(1) {
        info!("Running with prefix: {}", prefix);
        prefix + ENDO_DNA
    } else {
        info!("Running with empty prefix");
        ENDO_DNA.into()
    };
    let mut dna_executor = DnaExecutor::from(&prefix[..]);
    loop {
        let is_finished = dna_executor.execute_loops(1000);

        if is_finished {
            break;
        }
    }
    let dummy = [RNA::Unknown(vec![])];

    let rna = dna_executor.rna();

    info!("rna len: {}", rna.len());

    let mut renderer = RnaRenderer::new();
    let mut last_command = &RNA::Unknown(vec![]);
    let mut same_command_count = 0;

    for (index, command) in rna.iter().chain(dummy.iter()).enumerate() {
        if command == last_command {
            same_command_count += 1;
        } else {
            match last_command {
                RNA::Unknown(_) => (),
                _ => println!(
                    "{} {:?}{} {}",
                    index,
                    last_command,
                    if same_command_count == 1 {
                        "".into()
                    } else {
                        format!(" x {}", same_command_count)
                    },
                    match last_command {
                        RNA::TryFill => {
                            format!("{:?} {:?}", renderer.position(), renderer.current_pixel())
                        }
                        RNA::Move => format!("{:?}", renderer.position()),
                        RNA::Line => format!(
                            "{:?} {:?} {:?}",
                            renderer.mark(),
                            renderer.position(),
                            renderer.current_pixel()
                        ),
                        _ => format!(""),
                    }
                ),
            }
            last_command = command;
            same_command_count = 1;
        }
        renderer.render_command(command);
    }
}
