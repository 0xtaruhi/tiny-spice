use crate::netlist::Equation;
use sprs::CsVec;

pub trait Solver {
    fn solve(e: Equation) -> Result<CsVec<f64>, Box<dyn std::error::Error>>;
}
