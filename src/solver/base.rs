use crate::netlist::Netlist;
use sprs::CsVec;

pub trait Solver {
    fn solve(netlist: &Netlist) -> Result<CsVec<f64>, Box<dyn std::error::Error>>;
}
