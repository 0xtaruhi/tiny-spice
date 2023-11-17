use num_traits::{Num, NumOps};
use sprs::CsMat;

pub trait LUDecomp {
    type ResultType;
    fn lu_decomp(&self)
        -> Result<(Self::ResultType, Self::ResultType), Box<dyn std::error::Error>>;
}

impl<T> LUDecomp for CsMat<T>
where
    T: Clone + Default + PartialEq + PartialOrd + Copy + Num + NumOps,
{
    type ResultType = CsMat<T>;

    fn lu_decomp(
        &self,
    ) -> Result<(Self::ResultType, Self::ResultType), Box<dyn std::error::Error>> {
        assert!(self.cols() == self.rows(), "Matrix must be square");
        let size = self.rows();

        let mut l: CsMat<T> = CsMat::eye(size);
        let mut u: CsMat<T> = CsMat::empty(self.storage(), size);

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

        for s in 0..size {
            {
                // U
                let row = s;
                for col in row..size {
                    let orig_val = get_or_default(self.get(row, col));
                    let prev_sum = (0..row)
                        .map(|i| {
                            let l_val = get_or_default(l.get(row, i));
                            let u_val = get_or_default(u.get(i, col));
                            l_val * u_val
                        })
                        .fold(Default::default(), |acc: T, x| acc + x);

                    let u_val = orig_val - prev_sum;
                    u.insert(row, col, u_val);
                }
            }

            {
                // L
                let col = s;
                for row in (col + 1)..size {
                    let orig_val = get_or_default(self.get(row, col));
                    let prev_sum = (0..col)
                        .map(|i| {
                            let l_val = get_or_default(l.get(row, i));
                            let u_val = get_or_default(u.get(i, col));
                            l_val * u_val
                        })
                        .fold(Default::default(), |acc: T, x| acc + x);

                    let u_col_col = get_or_default(u.get(col, col));

                    if u_col_col == Default::default() {
                        return Err("LU decomposition failed".into());
                    }

                    let l_val = (orig_val - prev_sum) / u_col_col;
                    l.insert(row, col, l_val);
                }
            }
        }

        Ok((l, u))
    }
}
