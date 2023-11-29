use clap::Parser;
use std::cell::Cell;
use std::path::PathBuf;
use std::time::Instant;

use log::{error, info};

#[macro_use]
extern crate lazy_static;

mod analyze;
mod elements;
mod matrix;
mod netlist;
mod parser;
mod plot;
mod solver;
mod task;

#[derive(Parser, Debug)]
#[clap(author = "0xtaruhi", version, about)]
struct Opts {
    #[clap(short, long)]
    mode: Option<String>,

    #[clap(short, long)]
    disp: Option<usize>,

    #[clap(short, long)]
    final_time: Option<f64>,

    file: PathBuf,
}

fn run(opts: Opts) -> Result<(), Box<dyn std::error::Error>> {
    let parser = parser::Parser::new(opts.file);
    let parsed_info = parser.parse().map_err(|e| {
        error!("Failed to parse file: {}", e);
        e
    })?;
    info!("Parse successful");

    let tasks = parsed_info.tasks;
    let netlist = netlist::Netlist {
        node_num: Cell::new(parsed_info.node_num),
        basic_elements: parsed_info.basic_elements,
        time_varing_linear_elements: parsed_info.time_varing_linear_elements,
        time_varing_non_linear_elements: parsed_info.time_varing_non_linear_elements,
    };

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

    if let Some(t) = opts.final_time {
        if mode != analyze::Mode::Trans {
            return Err("Final time can only be specified in Trans mode".into());
        }
        analyzer.set_final_time(t);
    }

    info!("Solving...");

    analyzer.analyze(&tasks).map_err(|e| {
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

    fn dc_test(file: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
        let opts = Opts {
            mode: Some("dc".to_string()),
            disp: None,
            final_time: None,
            file,
        };
        run(opts)
    }

    fn trans_test(file: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
        let opts = Opts {
            mode: Some("trans".to_string()),
            disp: None,
            final_time: Some(1.),
            file,
        };
        run(opts)
    }

    #[test]
    fn test_examples2() -> Result<(), Box<dyn std::error::Error>> {
        let file = PathBuf::from("examples/test2.sp");
        dc_test(file)
    }

    #[test]
    fn test_examples3() -> Result<(), Box<dyn std::error::Error>> {
        let file = PathBuf::from("examples/test3.sp");
        dc_test(file)
    }

    #[test]
    fn test_examples4() -> Result<(), Box<dyn std::error::Error>> {
        let file = PathBuf::from("examples/test4.sp");
        dc_test(file)
    }

    #[test]
    fn test_inverter() -> Result<(), Box<dyn std::error::Error>> {
        let file = PathBuf::from("examples/inverter.sp");
        dc_test(file)
    }

    #[test]
    fn test_trans_example1() -> Result<(), Box<dyn std::error::Error>> {
        let file = PathBuf::from("examples/trans_test1.sp");
        trans_test(file)
    }

    #[test]
    fn test_trans_example2() -> Result<(), Box<dyn std::error::Error>> {
        let file = PathBuf::from("examples/trans_test2.sp");
        trans_test(file)
    }
}
