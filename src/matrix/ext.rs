use sprs::CsMat;

pub trait MatExt<T> {
    fn update_by_node_id(&mut self, row: usize, col: usize, val: T);

    fn add_by_node_id(&mut self, row: usize, col: usize, val: T);
}

pub trait VecExt<T> {
    fn update_by_node_id(&mut self, row: usize, val: T);

    fn add_by_node_id(&mut self, row: usize, val: T);

    fn get_by_node_id(&self, row: usize) -> T;
}

impl<T> MatExt<T> for CsMat<T>
where
    T: std::ops::AddAssign
        + std::clone::Clone
        + std::fmt::Debug
        + std::ops::MulAssign
        + std::ops::Neg<Output = T>
        + std::ops::SubAssign,
{
    fn update_by_node_id(&mut self, row: usize, col: usize, val: T) {
        if (row == 0) || (col == 0) {
            return;
        }
        assert!(row <= self.rows() && col <= self.cols());
        let ref_cell = self.get_mut(row - 1, col - 1).unwrap();
        *ref_cell = val;
    }

    fn add_by_node_id(&mut self, row: usize, col: usize, val: T) {
        if (row == 0) || (col == 0) {
            return;
        }
        assert!(row <= self.rows() && col <= self.cols());
        let ref_cell = self.get_mut(row - 1, col - 1).unwrap();
        *ref_cell += val;
    }
}

impl<T> VecExt<T> for sprs::CsVec<T>
where
    T: std::ops::AddAssign
        + std::clone::Clone
        + std::fmt::Debug
        + std::ops::MulAssign
        + std::ops::Neg<Output = T>
        + std::ops::SubAssign
        + num_traits::Zero,
{
    fn update_by_node_id(&mut self, row: usize, val: T) {
        if row == 0 {
            return;
        }
        assert!(row <= self.dim());
        let ref_cell = self.get_mut(row - 1).unwrap();
        *ref_cell = val;
    }

    fn add_by_node_id(&mut self, row: usize, val: T) {
        if row == 0 {
            return;
        }
        assert!(row <= self.dim());
        let ref_cell = self.get_mut(row - 1).unwrap();
        *ref_cell += val;
    }

    fn get_by_node_id(&self, row: usize) -> T {
        if row == 0 {
            return T::zero();
        }
        assert!(row <= self.dim());
        
        let ref_cell = self.get(row - 1);
        if ref_cell.is_none() {
            return T::zero();
        }
        ref_cell.unwrap().clone()
    }
}
