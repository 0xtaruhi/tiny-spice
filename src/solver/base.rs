use crate::elements::base::{NonLinearElement, TimeVaringNonLinearElement};
use sprs::{CsMat, CsVec};

pub trait Solver {
    fn solve_dc(
        mat: &CsMat<f64>,
        v: &CsVec<f64>,
        non_linear_elements: &[Box<dyn NonLinearElement>],
        time_varing_non_linear_elements: &[Box<dyn TimeVaringNonLinearElement>],
    ) -> Result<CsVec<f64>, Box<dyn std::error::Error>>;
}
