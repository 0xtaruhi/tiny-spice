use core::fmt;
use std::ops::Neg;

use log::debug;
use num_traits::{Num, NumOps};
use rayon::prelude::*;
use sprs::CsMat;
use std::sync::{Arc, RwLock};

pub trait LUDecomp {
    type ResultType;
    fn get_reorder_map(&self) -> Vec<usize>;

    fn lu_decomp(
        &self,
        reorder_map: Option<&Vec<usize>>,
    ) -> Result<(Self::ResultType, Self::ResultType), Box<dyn std::error::Error>>;
}

impl<T> LUDecomp for CsMat<T>
where
    T: Clone
        + Default
        + PartialEq
        + PartialOrd
        + Copy
        + Num
        + NumOps
        + fmt::Display
        + fmt::Debug
        + Neg<Output = T>
        + Send
        + Sync,
{
    type ResultType = CsMat<T>;

    fn get_reorder_map(&self) -> Vec<usize> {
        let mut reorder_map: Vec<usize> = (0..self.rows()).collect();

        for i in 0..self.rows() {
            fn abs<T>(x: T) -> T
            where
                T: PartialOrd + NumOps + Num + Copy + Neg<Output = T>,
            {
                if x < T::zero() {
                    -x
                } else {
                    x
                }
            }
            let mut max_row = i;

            {
                let get_mat_val = |row, col| get_or_default(self.get(reorder_map[row], col));
                let mut max_val = abs(get_mat_val(i, i));
                for j in (i + 1)..self.rows() {
                    let val = abs(get_mat_val(j, i));
                    if val > max_val {
                        max_val = val;
                        max_row = j;
                    }
                }
            }

            if max_row != i {
                reorder_map.swap(i, max_row);
            }
        }

        debug!("reorder_map: {:?}", reorder_map);

        reorder_map
    }

    fn lu_decomp(
        &self,
        reorder_map: Option<&Vec<usize>>,
    ) -> Result<(Self::ResultType, Self::ResultType), Box<dyn std::error::Error>> {
        assert!(self.cols() == self.rows(), "Matrix must be square");
        let size = self.rows();

        let l: Arc<RwLock<CsMat<T>>> = Arc::new(RwLock::new(CsMat::eye(size)));
        let u: Arc<RwLock<CsMat<T>>> = Arc::new(RwLock::new(CsMat::empty(self.storage(), size)));

        let get_self_mat_val = |row, col| {
            if let Some(m) = &reorder_map {
                self.get(m[row], col)
            } else {
                self.get(row, col)
            }
        };

        for s in 0..size {
            {
                // U
                let row = s;
                for col in row..size {
                    let orig_val = get_or_default(get_self_mat_val(row, col));
                    let prev_sum = (0..row)
                        .into_par_iter()
                        .map(|i| {
                            let l_val = get_or_default(l.read().unwrap().get(row, i));
                            let u_val = get_or_default(u.read().unwrap().get(i, col));
                            l_val * u_val
                        })
                        .reduce(|| Default::default(), |acc: T, x| acc + x);

                    let u_val = orig_val - prev_sum;
                    u.write().unwrap().insert(row, col, u_val);

                }
            }

            {
                // L
                let col = s;
                // for row in ((col + 1)..size).into_iter() {
                ((col + 1)..size).into_par_iter().for_each(|row| {
                    let orig_val = get_or_default(get_self_mat_val(row, col));
                    let prev_sum = (0..col)
                        .into_par_iter()
                        .map(|i| {
                            let l_val = get_or_default(l.read().unwrap().get(row, i));
                            let u_val = get_or_default(u.read().unwrap().get(i, col));
                            l_val * u_val
                        })
                        .reduce(|| Default::default(), |acc: T, x| acc + x);

                    let u_col_col = get_or_default(u.read().unwrap().get(col, col));

                    if u_col_col == Default::default() {
                        panic!("Matrix is singular");
                    }

                    let l_val = (orig_val - prev_sum) / u_col_col;
                    l.write().unwrap().insert(row, col, l_val);
                });
            }
        }

        let l = Arc::try_unwrap(l).unwrap().into_inner().unwrap();
        let u = Arc::try_unwrap(u).unwrap().into_inner().unwrap();
        // debug!("\nL:{}\nU:{}\n", l.to_dense(), u.to_dense());

        Ok((l, u))
    }
}

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
