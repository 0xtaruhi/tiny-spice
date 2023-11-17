use super::base::{BasicComponent, Component, ComponentType};
use crate::netlist::NodeId;

#[derive(Debug)]
pub struct Resistor {
    name: String,
    node_in: NodeId,
    node_out: NodeId,
    value: f64,
}

impl Resistor {
    pub fn new(name: String, node_in: NodeId, node_out: NodeId, value: f64) -> Self {
        Self {
            name,
            node_in,
            node_out,
            value,
        }
    }

    pub fn parse(s: &str) -> Self {
        let mut iter = s.split_whitespace();
        let name = iter.next().unwrap().to_string();
        let node_in = iter.next().unwrap().parse::<NodeId>().unwrap();
        let node_out = iter.next().unwrap().parse::<NodeId>().unwrap();
        let value = iter.next().unwrap().parse::<f64>().unwrap();
        Self::new(name, node_in, node_out, value)
    }
}

impl Component for Resistor {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_type(&self) -> ComponentType {
        ComponentType::Resistor
    }
}

impl BasicComponent for Resistor {
    fn get_node_in(&self) -> NodeId {
        self.node_in
    }

    fn get_node_out(&self) -> NodeId {
        self.node_out
    }

    fn get_base_value(&self) -> f64 {
        self.value
    }

    fn set_matrix_dc(&self, mat: &mut crate::matrix::build::MatrixTriplets<f64>, _v: &mut crate::matrix::build::VecItems<f64>) {
        let g = 1. / self.get_base_value();

        let (node_in, node_out) = (self.get_node_in(), self.get_node_out());

        mat.push_with_node_id(node_in, node_in, g);
        mat.push_with_node_id(node_in, node_out, -g);
        mat.push_with_node_id(node_out, node_in, -g);
        mat.push_with_node_id(node_out, node_out, g);
    }
}
