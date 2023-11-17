use super::base::{Element, ElementType, MatrixSettable, NonLineaerTwoPortElement, TwoPortElement};
use crate::netlist::NodeId;

#[derive(Debug)]
pub struct Capacitor {
    name: String,
    node_in: NodeId,
    node_out: NodeId,
    value: f64,
}
impl Capacitor {
    pub fn new(name: String, node_in: NodeId, node_out: NodeId, value: f64) -> Self {
        Self {
            name,
            node_in,
            node_out,
            value,
        }
    }

    pub fn parse(s: &str) -> Self {
        let (name, node_in, node_out, value) = super::base::general_linear_element_parse(s);
        Self::new(name, node_in, node_out, value)
    }
}

impl Element for Capacitor {
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_type(&self) -> ElementType {
        ElementType::Capacitor
    }
}

impl TwoPortElement for Capacitor {
    fn get_node_in(&self) -> NodeId {
        self.node_in
    }

    fn get_node_out(&self) -> NodeId {
        self.node_out
    }
}

impl NonLineaerTwoPortElement for Capacitor {
    fn get_base_value(&self) -> f64 {
        self.value
    }
}

impl MatrixSettable for Capacitor {
    fn set_matrix_dc(
        &self,
        _mat: &mut crate::matrix::build::MatrixTriplets<f64>,
        _v: &mut crate::matrix::build::VecItems<f64>,
    ) {
    }
}