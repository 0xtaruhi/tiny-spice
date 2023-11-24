use super::base::{Element, MatrixSettable};
use crate::matrix::build::VecPushWithNodeId;
use crate::netlist::NodeId;

#[derive(Debug)]
enum SourceType {
    AC,
    DC,
}

#[derive(Debug)]
enum BasicElementType {
    Resistor(f64),
    VoltageSource(SourceType, f64),
    CurrentSource(SourceType, f64),
}

#[derive(Debug)]
pub struct BasicElement {
    name: String,
    node_in: NodeId,
    node_out: NodeId,
    element_type: BasicElementType,
}

impl Element for BasicElement {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_nodes(&self) -> Vec<NodeId> {
        vec![self.node_in, self.node_out]
    }
}

impl BasicElement {
    fn get_node_in(&self) -> NodeId {
        self.node_in
    }

    fn get_node_out(&self) -> NodeId {
        self.node_out
    }

    fn get_base_value(&self) -> f64 {
        match self.element_type {
            BasicElementType::Resistor(value) => value,
            BasicElementType::VoltageSource(_, value) => value,
            BasicElementType::CurrentSource(_, value) => value,
        }
    }
}

impl BasicElement {
    fn set_matrix_dc_resistor(
        &self,
        mat: &mut crate::matrix::build::MatrixTriplets<f64>,
        _v: &mut crate::matrix::build::VecItems<f64>,
    ) {
        let g = 1. / self.get_base_value();

        let (node_in, node_out) = (self.get_node_in(), self.get_node_out());

        mat.push_with_node_id(node_in, node_in, g);
        mat.push_with_node_id(node_in, node_out, -g);
        mat.push_with_node_id(node_out, node_in, -g);
        mat.push_with_node_id(node_out, node_out, g);
    }

    fn set_matrix_dc_voltage_source(
        &self,
        mat: &mut crate::matrix::build::MatrixTriplets<f64>,
        v: &mut crate::matrix::build::VecItems<f64>,
    ) {
        let new_pos = mat.size;
        mat.extend_size(1);

        let (node_in, node_out) = (self.get_node_in(), self.get_node_out());

        mat.push_with_node_id(new_pos + 1, node_in, 1.);
        mat.push_with_node_id(new_pos + 1, node_out, -1.);
        mat.push_with_node_id(node_in, new_pos + 1, 1.);
        mat.push_with_node_id(node_out, new_pos + 1, -1.);

        v.insert(new_pos, self.get_base_value());
    }

    fn set_matrix_dc_current_source(
        &self,
        _mat: &mut crate::matrix::build::MatrixTriplets<f64>,
        v: &mut crate::matrix::build::VecItems<f64>,
    ) {
        let (node_in, node_out) = (self.get_node_in(), self.get_node_out());
        v.push_with_node_id(node_in, -self.get_base_value());
        v.push_with_node_id(node_out, self.get_base_value());
    }
}

impl MatrixSettable for BasicElement {
    fn set_matrix_dc(
        &self,
        mat: &mut crate::matrix::build::MatrixTriplets<f64>,
        v: &mut crate::matrix::build::VecItems<f64>,
    ) {
        match self {
            BasicElement {
                element_type: BasicElementType::Resistor(_),
                ..
            } => self.set_matrix_dc_resistor(mat, v),
            BasicElement {
                element_type: BasicElementType::VoltageSource(_, _),
                ..
            } => self.set_matrix_dc_voltage_source(mat, v),
            BasicElement {
                element_type: BasicElementType::CurrentSource(_, _),
                ..
            } => self.set_matrix_dc_current_source(mat, v),
        }
    }
}

impl BasicElement {
    pub fn parse_resistor(s: &str) -> Option<Self> {
        let (name, node_in, node_out, val) = super::base::general_element_parse(s)?;
        Some(Self {
            name,
            node_in,
            node_out,
            element_type: BasicElementType::Resistor(val),
        })
    }

    pub fn parse_voltage_source(s: &str) -> Option<Self> {
        let mut iter = s.split_whitespace();
        let name = iter.next()?.to_string();
        let node_in = iter.next()?.parse::<NodeId>().unwrap();
        let node_out = iter.next()?.parse::<NodeId>().unwrap();
        let source_type = match iter.next()? {
            "DC" => SourceType::DC,
            "AC" => SourceType::AC,
            _ => return None,
        };
        let value = iter.next()?.parse::<f64>().unwrap();

        Some(Self {
            name,
            node_in,
            node_out,
            element_type: BasicElementType::VoltageSource(source_type, value),
        })
    }

    pub fn parse_current_source(s: &str) -> Option<Self> {
        let mut iter = s.split_whitespace();
        let name = iter.next()?.to_string();
        let node_in = iter.next()?.parse::<NodeId>().unwrap();
        let node_out = iter.next()?.parse::<NodeId>().unwrap();
        let source_type = match iter.next()? {
            "DC" => SourceType::DC,
            "AC" => SourceType::AC,
            _ => return None,
        };
        let value = iter.next()?.parse::<f64>().unwrap();

        Some(Self {
            name,
            node_in,
            node_out,
            element_type: BasicElementType::CurrentSource(source_type, value),
        })
    }
}
