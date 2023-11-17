use crate::netlist::NodeId;

pub struct MatrixTriplets<T> {
    pub rows: Vec<usize>,
    pub cols: Vec<usize>,
    pub vals: Vec<T>,
    pub size: usize,
}

pub struct VecItems<T> {
    pub idxs: Vec<usize>,
    pub vals: Vec<T>,
}

impl<T> VecItems<T> {
    pub fn new() -> Self {
        VecItems {
            idxs: Vec::new(),
            vals: Vec::new(),
        }
    }

    pub fn push(&mut self, index: usize, val: T) {
        self.idxs.push(index);
        self.vals.push(val);
    }
}

impl<T> MatrixTriplets<T> {
    pub fn new(size: usize) -> MatrixTriplets<T> {
        MatrixTriplets {
            rows: Vec::new(),
            cols: Vec::new(),
            vals: Vec::new(),
            size: size,
        }
    }

    #[allow(dead_code)]
    pub fn push(&mut self, row: usize, col: usize, val: T) {
        self.rows.push(row);
        self.cols.push(col);
        self.vals.push(val);
    }

    pub fn push_with_node_id(&mut self, row: NodeId, col: NodeId, val: T) {
        if row == 0 || col == 0 {
            return;
        }
        self.rows.push(row - 1);
        self.cols.push(col - 1);
        self.vals.push(val);
    }

    pub fn extend_size(&mut self, x: usize) {
        self.size += x;
    }
}
