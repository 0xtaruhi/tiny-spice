use super::netlist::Netlist;
use super::solver::base::Solver;
use super::solver::newton::NewtonSolver;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    DC,
    Trans,
    Unknown,
}

impl Into<Mode> for String {
    fn into(self) -> Mode {
        match self.to_ascii_uppercase().as_str() {
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

    pub fn analyze(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self.config.mode {
            Mode::DC => self.analyze_dc(),
            Mode::Trans => self.analyze_trans(),
            Mode::Unknown => {
                panic!("Unknown mode")
            }
        }
    }

    fn analyze_dc(&self) -> Result<(), Box<dyn std::error::Error>> {
        let result = NewtonSolver::solve(self.netlist.get_equation())?;
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

    fn analyze_trans(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
