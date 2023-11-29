use log::debug;
use sprs::CsVec;

use crate::elements::base::MatrixTransUpdatable;
use crate::elements::companion::CompanionModel;
use crate::task::{Task, TaskResult};

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
    final_time: f64,
}

impl Default for AnalyzerConfig {
    fn default() -> Self {
        Self {
            mode: Mode::Unknown,
            disp_digits: 5,
            final_time: 10.,
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

    pub fn set_final_time(&mut self, final_time: f64) {
        self.config.final_time = final_time;
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
        let e: crate::netlist::Equation = self.netlist.get_equation_dc();
        let time_varing_non_linear_elements = &self.netlist.time_varing_non_linear_elements;

        let result = NewtonSolver::solve_dc(
            &e.mat_a,
            &e.vec_b,
            time_varing_non_linear_elements.as_slice(),
        )?;
        let node_num = self.netlist.node_num.get();

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

    fn analyze_trans(&self, tasks: &[Task]) -> Result<(), Box<dyn std::error::Error>> {
        let delta_t = 1e-2;
        let final_time = self.config.final_time;

        let mut companion_models = self
            .netlist
            .time_varing_linear_elements
            .iter()
            .map(|e| CompanionModel::new_from_linear(e, &self.netlist))
            .collect::<Vec<_>>();

        let basic_eq = self.netlist.get_equation_trans(&companion_models);
        let (basic_mat_a, basic_vec_b) = (basic_eq.mat_a, basic_eq.vec_b);

        let mut x = CsVec::empty(basic_vec_b.dim());
        let mut current_time = 0.;

        let mut time_stamps = Vec::new();
        let mut task_results = tasks.iter().map(TaskResult::new).collect::<Vec<_>>();

        while current_time < final_time {
            let mut mat_a = basic_mat_a.clone();
            let mut vec_b = basic_vec_b.clone();

            companion_models.iter_mut().for_each(|m| {
                m.update_companion_elements(&x, delta_t);
                m.update_matrix_trans(&mut mat_a, &mut vec_b, &x);
            });

            debug!("mat_a: {}", mat_a.to_dense());
            debug!("vec_b: {}", vec_b.to_dense());

            x = NewtonSolver::solve_dc(
                &mat_a,
                &vec_b,
                self.netlist.time_varing_non_linear_elements.as_slice(),
            )?;

            debug!("x: {}", x.to_dense());

            time_stamps.push(current_time);
            for task in &mut task_results {
                task.update(&x);
            }

            current_time += delta_t;
            companion_models.iter_mut().for_each(|m| {
                m.update_current(&x);
            });
        }

        task_results.iter().for_each(|task| {
            task.run(&time_stamps);
        });

        Ok(())
    }
}
