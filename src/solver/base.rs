use crate::elements::TimeVaringNonLinearElement;
use sprs::{CsMat, CsVec};

pub trait Solver {
    fn solve_dc(
        mat: &CsMat<f64>,
        v: &CsVec<f64>,
        time_varing_non_linear_elements: &[TimeVaringNonLinearElement],
    ) -> Result<CsVec<f64>, Box<dyn std::error::Error>>;
}
