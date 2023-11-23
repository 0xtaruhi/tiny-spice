use crate::{matrix::decomp::LUDecomp, netlist::Netlist};
use log::{error, info};
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
    fn solve(netlist: &Netlist) -> Result<sprs::CsVec<f64>, Box<dyn std::error::Error>> {
        let e = netlist.get_equation();
        let basic_mat_a = e.mat_a;
        let basic_vec_b = e.vec_b;

        let mut x = CsVec::empty(netlist.node_num - 1);

        let mut iter_times = 0;

        loop {
            iter_times += 1;
            let mut mat_a = basic_mat_a.clone();
            let mut vec_b = basic_vec_b.clone();

            for non_linear_element in &netlist.non_linear_elements {
                non_linear_element.update_matrix_dc(&mut mat_a, &mut vec_b, &x);
            }

            let x_next = LUSolver::solve(&mat_a, &vec_b)?;

            let l2_diff = {
                let mut diff_acc = 0.;
                for i in  0..(netlist.node_num - 1) {
                    let diff = get_or_default(x.get(i)) - get_or_default(x_next.get(i));
                    diff_acc += diff * diff;
                }
                diff_acc
            };

            x = x_next;

            if l2_diff < 1e-6 {
                break;
            }

            if iter_times > MAX_ITER {
                return Err("Newton method failed to converge".into());
            }
        }

        info!("Newton method converged in {} iterations", iter_times);
        Ok(x)
    }
}
