use crate::{
    matrix::build::{MatrixTriplets, VecItems},
    netlist::NodeId,
};

use sprs::{CsMat, CsVec};

pub enum ElementType {
    Resistor,
    VoltageSource,
    CurrentSource,
    Capacitor,
    Inductor,
    Mosfet,
}

pub trait Element {
    fn get_name(&self) -> &str;

    fn get_type(&self) -> ElementType;

    fn get_nodes(&self) -> Vec<NodeId>;
}

pub trait TwoPortElement: Element {
    fn get_node_in(&self) -> NodeId;

    fn get_node_out(&self) -> NodeId;

    fn get_base_value(&self) -> f64;
}

pub trait LinearElement: TwoPortElement + MatrixSettable {}

pub trait MatrixSettable {
    fn set_matrix_dc(&self, mat: &mut MatrixTriplets<f64>, v: &mut VecItems<f64>);
}

pub trait MatrixUpdatable {
    fn update_matrix_dc(&self, mat: &mut CsMat<f64>, v: &mut CsVec<f64>, x: &CsVec<f64>);
}

pub trait NonLinearElement: Element + MatrixSettable + MatrixUpdatable {}

pub fn general_linear_element_parse(s: &str) -> (String, NodeId, NodeId, f64) {
    let mut iter = s.split_whitespace();
    let name = iter.next().unwrap().to_string();
    let node_in = iter.next().unwrap().parse::<NodeId>().unwrap();
    let node_out = iter.next().unwrap().parse::<NodeId>().unwrap();
    let value = iter.next().unwrap().parse::<f64>().unwrap();

    (name, node_in, node_out, value)
}
