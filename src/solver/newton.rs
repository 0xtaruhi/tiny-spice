use crate::{matrix::decomp::LUDecomp, netlist::Equation};
use log::error;
use sprs::{CsMat, CsVec};

use super::base::Solver;

pub struct NewtonSolver {}

const MAX_ITER: usize = 100;

fn get_or_default<T>(x: Option<&T>) -> T
where
    T: Default + Clone,
{
    if let Some(val) = x {
        val.clone()
    } else {
        Default::default()
    }
}

struct LUSolver {}

impl LUSolver {
    fn solve(mat: &CsMat<f64>, v: &CsVec<f64>) -> Result<CsVec<f64>, Box<dyn std::error::Error>> {
        let reorder_map = mat.get_reorder_map();

        let (l, u) = mat.lu_decomp(Some(&reorder_map)).map_err(|e| {
            error!("LU decomposition failed: {}", e);
            e
        })?;

        assert!(mat.rows() == mat.cols());
        let size = mat.rows();

        let b_star = {
            let mut result: CsVec<f64> = CsVec::empty(size);
            for row in 0..size {
                let prev_sum = (0..row)
                    .map(|i| {
                        let l_val = get_or_default(l.get(row, i));
                        let result_val = get_or_default(result.get(i));
                        l_val * result_val
                    })
                    .reduce(|acc, x| acc + x);

                let prev_sum = if prev_sum.is_some() {
                    prev_sum.unwrap()
                } else {
                    0.0
                };
                result.append(
                    row,
                    (get_or_default(v.get(reorder_map[row])) - prev_sum)
                        / get_or_default(l.get(row, row)),
                );
            }
            result
        };

        let x = {
            let mut vals = vec![0.0; size];

            for row in (0..size).rev() {
                let prev_sum = (row + 1..size)
                    .map(|i| {
                        let u_val = get_or_default(u.get(row, i));
                        let result_val = vals[i];
                        u_val * result_val
                    })
                    .reduce(|acc, x| acc + x);

                let prev_sum = if prev_sum.is_some() {
                    prev_sum.unwrap()
                } else {
                    0.0
                };

                vals[row] =
                    (get_or_default(b_star.get(row)) - prev_sum) / get_or_default(u.get(row, row));
            }

            let result: CsVec<f64> = CsVec::new(
                size,
                (0..size).collect::<Vec<usize>>(),
                (0..size).map(|i| vals[i]).collect::<Vec<f64>>(),
            );

            result
        };

        Ok(x)
    }
}

impl Solver for NewtonSolver {
    fn solve(e: Equation) -> Result<sprs::CsVec<f64>, Box<dyn std::error::Error>> {
        let mat_a = e.mat_a.clone();
        let vec_b = e.vec_b.clone();

        let x = LUSolver::solve(&mat_a, &vec_b);

        // let size = mat_a.cols();

        // let mut x = sprs::CsVec::empty(size);
        // let mut r = vec_b.clone().sub(&(&mat_a * &x));

        // let mut i = 0;

        // while r.l2_norm() > 1e-6 && i < MAX_ITER {
        //     let d = mat_a.diag();
        //     let d_inv = d.map(|x| 1.0 / x);

        //     let mut delta_x = sprs::CsVec::empty(size);
        //     for (index, value) in d_inv.iter() {
        //         if let Some(other_value) = r.get(index) {
        //             delta_x.append(index, value * other_value);
        //         }
        //     }

        //     x = x + &delta_x;
        //     r = vec_b.clone().sub(&(&mat_a * &x));
        //     i += 1;
        // }

        // println!("{} iterations", i);

        x
    }
}
