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

    fn get_base_value(&self) -> f64 {
        self.value
    }
}

impl BasicComponent for Resistor {
    fn get_node_in(&self) -> NodeId {
        self.node_in
    }

    fn get_node_out(&self) -> NodeId {
        self.node_out
    }
}
