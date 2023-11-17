use super::base::{BasicComponent, Component, ComponentType};
use crate::netlist::NodeId;

#[derive(Debug)]
pub enum VoltageSourceType {
    DC(f64),
    AC(f64, f64),
}

#[derive(Debug)]
pub struct VoltageSource {
    name: String,
    node_in: NodeId,
    node_out: NodeId,
    source_type: VoltageSourceType,
}

impl VoltageSource {
    pub fn new(
        name: String,
        node_in: NodeId,
        node_out: NodeId,
        source_type: VoltageSourceType,
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
            "DC" => VoltageSourceType::DC(iter.next().unwrap().parse::<f64>().unwrap()),
            "AC" => VoltageSourceType::AC(
                iter.next().unwrap().parse::<f64>().unwrap(),
                iter.next().unwrap().parse::<f64>().unwrap(),
            ),
            _ => panic!("Invalid voltage source type"),
        };
        Self::new(name, node_in, node_out, source_type)
    }
}

impl Component for VoltageSource {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_type(&self) -> ComponentType {
        ComponentType::VoltageSource
    }

    fn get_base_value(&self) -> f64 {
        match self.source_type {
            VoltageSourceType::DC(v) => v,
            VoltageSourceType::AC(v, _) => v,
        }
    }
}

impl BasicComponent for VoltageSource {
    fn get_node_in(&self) -> NodeId {
        self.node_in
    }

    fn get_node_out(&self) -> NodeId {
        self.node_out
    }
}