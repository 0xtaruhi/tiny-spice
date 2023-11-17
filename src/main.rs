use clap::Parser;
use std::path::PathBuf;
use std::time::Instant;

use log::{error, info};

mod analyze;
mod elements;
mod matrix;
mod netlist;
mod parser;
mod solver;
mod task;

#[derive(Parser, Debug)]
#[clap(author = "0xtaruhi", version, about)]
struct Opts {
    #[clap(short, long)]
    mode: Option<String>,

    #[clap(short, long)]
    disp: Option<usize>,

    file: PathBuf,
}

fn run(opts: Opts) -> Result<(), Box<dyn std::error::Error>> {
    let parser = parser::Parser::new(opts.file);
    let parsed_info = parser.parse().map_err(|e| {
        error!("Failed to parse file: {}", e);
        e
    })?;

    info!("Parse successful");

    let netlist = netlist::Netlist::new(parsed_info);

    let mode: analyze::Mode = {
        if let Some(m) = opts.mode {
            m.into()
        } else {
            analyze::Mode::DC
        }
    };

    let mut analyzer = analyze::Analyzer::new(netlist);
    analyzer.set_mode(mode);
    if let Some(d) = opts.disp {
        analyzer.set_disp_digits(d);
    }

    info!("Solving...");

    analyzer.analyze().map_err(|e| {
        error!("Failed to analyze: {}", e);
        e
    })?;
    info!("Analysis successful");
    Ok(())
}

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let opts = Opts::parse();
    if !opts.file.exists() {
        panic!("File does not exist!");
    }

    let start = Instant::now();
    run(opts).expect("Failed to run");
    let elapsed = start.elapsed();

    info!("Elapsed: {:.2?}", elapsed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples2() -> Result<(), Box<dyn std::error::Error>> {
        let file = PathBuf::from("examples/test2.sp");
        let opts = Opts {
            mode: Some("dc".to_string()),
            disp: None,
            file,
        };
        run(opts)
    }
}
