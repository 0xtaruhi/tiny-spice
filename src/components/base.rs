use crate::{netlist::NodeId, matrix::build::{MatrixTriplets, VecItems}};

pub enum ComponentType {
    Resistor,
    VoltageSource,
    CurrentSource,
    Capacitor,
}

pub trait Component {
    fn get_name(&self) -> &str;

    fn get_type(&self) -> ComponentType;

}

pub trait BasicComponent: Component {
    fn get_node_in(&self) -> NodeId;

    fn get_node_out(&self) -> NodeId;
    
    fn get_base_value(&self) -> f64;

    fn set_matrix_dc(&self, mat: &mut MatrixTriplets<f64>, v: &mut VecItems<f64>);
}

pub fn basic_component_parse(s: &str) -> (String, NodeId, NodeId, f64) {
    let mut iter = s.split_whitespace();
    let name = iter.next().unwrap().to_string();
    let node_in = iter.next().unwrap().parse::<NodeId>().unwrap();
    let node_out = iter.next().unwrap().parse::<NodeId>().unwrap();
    let value = iter.next().unwrap().parse::<f64>().unwrap();
    (name, node_in, node_out, value)
}
