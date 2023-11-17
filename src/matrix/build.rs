use crate::components::base::{BasicComponent, Component};
use crate::components::resistor::Resistor;
use crate::components::voltage_source::VoltageSource;
use crate::netlist::NodeId;

pub trait MatrixUpdatableComponent: Component {
    // fn update(&self) -> (MatrixTriplets<f64>, Vec<VecItem<f64>>);
    fn update(&self, mat: &mut MatrixTriplets<f64>, v: &mut VecItems<f64>);
}

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
        VecItems { idxs: Vec::new(), vals: Vec::new() }
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

impl MatrixUpdatableComponent for Resistor {
    fn update(&self, mat: &mut MatrixTriplets<f64>, _v: &mut VecItems<f64>) {
        let g = 1. / self.get_base_value();

        let (node_in, node_out) = (self.get_node_in(), self.get_node_out());

        mat.push_with_node_id(node_in, node_in, g);
        mat.push_with_node_id(node_in, node_out, -g);
        mat.push_with_node_id(node_out, node_in, -g);
        mat.push_with_node_id(node_out, node_out, g);
    }
}

impl MatrixUpdatableComponent for VoltageSource {
    fn update(&self, mat: &mut MatrixTriplets<f64>, v: &mut VecItems<f64>) {
        let new_pos = mat.size;
        mat.extend_size(1);

        let (node_in, node_out) = (self.get_node_in(), self.get_node_out());

        mat.push_with_node_id(new_pos + 1, node_in, 1.);
        mat.push_with_node_id(new_pos + 1, node_out, -1.);
        mat.push_with_node_id(node_in, new_pos + 1, 1.);
        mat.push_with_node_id(node_out, new_pos + 1, -1.);

        v.push(new_pos, self.get_base_value());
    }
}
