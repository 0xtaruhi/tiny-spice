use crate::{
    matrix::build::{MatrixTriplets, VecItems},
    netlist::NodeId,
};

use sprs::{CsMat, CsVec};

pub trait Element: MatrixSettable {
    fn get_name(&self) -> &str;

    fn get_nodes(&self) -> Vec<NodeId>;
}

pub trait TwoPortElement: Element {
    fn get_node_in(&self) -> NodeId;

    fn get_node_out(&self) -> NodeId;

    fn get_base_value(&self) -> f64;
}

pub trait MatrixSettable {
    fn set_matrix_dc(&self, mat: &mut MatrixTriplets<f64>, v: &mut VecItems<f64>);

    fn set_matrix_trans(&self, mat: &mut MatrixTriplets<f64>, v: &mut VecItems<f64>) {
        self.set_matrix_dc(mat, v);
    }
}

pub trait MatrixDcUpdatable {
    fn update_matrix_dc(&self, mat: &mut CsMat<f64>, v: &mut CsVec<f64>, x: &CsVec<f64>);
}

pub trait MatrixTransUpdatable {
    fn update_matrix_trans(&self, mat: &mut CsMat<f64>, v: &mut CsVec<f64>, x: &CsVec<f64>);
}

pub trait NonLinearElement: Element + MatrixDcUpdatable {}

pub trait TimeVaringNonLinearElement: Element + MatrixTransUpdatable + MatrixDcUpdatable {}

pub fn general_element_parse(s: &str) -> Option<(String, NodeId, NodeId, f64)> {
    let mut iter = s.split_whitespace();

    let name = iter.next()?.to_string();
    let node_in = iter.next()?.parse::<NodeId>().unwrap();
    let node_out = iter.next()?.parse::<NodeId>().unwrap();
    let value = iter.next()?.parse::<f64>().unwrap();

    Some((name, node_in, node_out, value))
}
