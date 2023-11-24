use crate::task::Task;

use super::netlist::Netlist;
use super::solver::base::Solver;
use super::solver::newton::NewtonSolver;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    DC,
    Trans,
    Unknown,
}

impl From<String> for Mode {
    fn from(val: String) -> Self {
        match val.to_ascii_uppercase().as_str() {
            "D" | "DC" => Mode::DC,
            "T" | "TRANS" => Mode::Trans,
            _ => Mode::Unknown,
        }
    }
}

struct AnalyzerConfig {
    mode: Mode,
    disp_digits: usize,
}

impl Default for AnalyzerConfig {
    fn default() -> Self {
        Self {
            mode: Mode::Unknown,
            disp_digits: 5,
        }
    }
}

pub struct Analyzer {
    netlist: Netlist,
    config: AnalyzerConfig,
}

impl Analyzer {
    pub fn new(netlist: Netlist) -> Self {
        Self {
            netlist,
            config: Default::default(),
        }
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.config.mode = mode;
    }

    pub fn set_disp_digits(&mut self, disp_digits: usize) {
        self.config.disp_digits = disp_digits;
    }

    pub fn analyze(&self, tasks: &[Task]) -> Result<(), Box<dyn std::error::Error>> {
        match self.config.mode {
            Mode::DC => self.analyze_dc(tasks),
            Mode::Trans => self.analyze_trans(tasks),
            Mode::Unknown => {
                panic!("Unknown mode")
            }
        }
    }

    fn analyze_dc(&self, _tasks: &[Task]) -> Result<(), Box<dyn std::error::Error>> {
        let e = self.netlist.get_equation_dc();
        let time_varing_non_linear_elements = &self.netlist.time_varing_non_linear_elements;

        let result = NewtonSolver::solve_dc(
            &e.mat_a,
            &e.vec_b,
            time_varing_non_linear_elements.as_slice(),
        )?;
        let node_num = self.netlist.node_num;

        for node_id in 0..(node_num - 1) {
            println!(
                "Node[{}]: {:.width$} V",
                node_id + 1,
                result[node_id],
                width = self.config.disp_digits
            );
        }

        Ok(())
    }

    fn analyze_trans(&self, _tasks: &[Task]) -> Result<(), Box<dyn std::error::Error>> {
        let _e = self.netlist.get_equation_trans();

        Ok(())
    }
}
