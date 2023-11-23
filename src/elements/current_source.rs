use super::base::{Element, ElementType, BasicElement, MatrixSettable, TwoPortElement};
use crate::matrix::build::VecPushWithNodeId;
use crate::netlist::NodeId;

#[derive(Debug)]
pub enum CurrentSourceType {
    DC(f64),
    AC(f64, f64),
}

pub struct CurrentSource {
    name: String,
    node_in: NodeId,
    node_out: NodeId,
    source_type: CurrentSourceType,
}

impl CurrentSource {
    pub fn new(
        name: String,
        node_in: NodeId,
        node_out: NodeId,
        source_type: CurrentSourceType,
    ) -> Self {
        Self {
            name,
            node_in,
            node_out,
            source_type,
        }
    }

    pub fn parse(s: &str) -> Self {
        let mut iter = s.split_whitespace();
        let name = iter.next().unwrap().to_string();
        let node_in = iter.next().unwrap().parse::<NodeId>().unwrap();
        let node_out = iter.next().unwrap().parse::<NodeId>().unwrap();
        let source_type = match iter.next().unwrap() {
            "DC" => CurrentSourceType::DC(iter.next().unwrap().parse::<f64>().unwrap()),
            "AC" => CurrentSourceType::AC(
                iter.next().unwrap().parse::<f64>().unwrap(),
                iter.next().unwrap().parse::<f64>().unwrap(),
            ),
            _ => panic!("Invalid current source type"),
        };
        Self::new(name, node_in, node_out, source_type)
    }
}

impl Element for CurrentSource {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_type(&self) -> ElementType {
        ElementType::CurrentSource
    }

    fn get_nodes(&self) -> Vec<NodeId> {
        vec![self.node_in, self.node_out]
    }
}

impl TwoPortElement for CurrentSource {
    fn get_node_in(&self) -> NodeId {
        self.node_in
    }

    fn get_node_out(&self) -> NodeId {
        self.node_out
    }
    fn get_base_value(&self) -> f64 {
        match self.source_type {
            CurrentSourceType::DC(v) => v,
            CurrentSourceType::AC(v, _) => v,
        }
    }
}

impl MatrixSettable for CurrentSource {
    fn set_matrix_dc(
        &self,
        _mat: &mut crate::matrix::build::MatrixTriplets<f64>,
        v: &mut crate::matrix::build::VecItems<f64>,
    ) {
        let (node_in, node_out) = (self.get_node_in(), self.get_node_out());
        v.push_with_node_id(node_in, -self.get_base_value());
        v.push_with_node_id(node_out, self.get_base_value());
    }
}

impl BasicElement for CurrentSource {}
