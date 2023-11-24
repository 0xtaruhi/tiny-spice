use crate::netlist::NodeId;

use super::base::{Element, MatrixSettable};

#[derive(Debug)]
enum TimeVaringLinearElementType {
    Capacitor(f64),
    Inductor(f64),
}

#[derive(Debug)]
pub struct TimeVaringLinearElement {
    name: String,
    node_in: NodeId,
    node_out: NodeId,
    element_type: TimeVaringLinearElementType,
}

impl Element for TimeVaringLinearElement {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_nodes(&self) -> Vec<NodeId> {
        vec![self.node_in, self.node_out]
    }
}

impl TimeVaringLinearElement {
    fn get_node_in(&self) -> NodeId {
        self.node_in
    }

    fn get_node_out(&self) -> NodeId {
        self.node_out
    }

    #[allow(dead_code)]
    fn get_base_value(&self) -> f64 {
        match self.element_type {
            TimeVaringLinearElementType::Capacitor(value) => value,
            TimeVaringLinearElementType::Inductor(value) => value,
        }
    }
}

impl TimeVaringLinearElement {
    fn set_matrix_dc_capacitor(
        &self,
        _mat: &mut crate::matrix::build::MatrixTriplets<f64>,
        _v: &mut crate::matrix::build::VecItems<f64>,
    ) {
    }

    fn set_matrix_dc_inductor(
        &self,
        mat: &mut crate::matrix::build::MatrixTriplets<f64>,
        _v: &mut crate::matrix::build::VecItems<f64>,
    ) {
        let new_pos = mat.size;
        mat.extend_size(1);

        let (node_in, node_out) = (self.get_node_in(), self.get_node_out());

        mat.push_with_node_id(new_pos + 1, node_in, 1.);
        mat.push_with_node_id(new_pos + 1, node_out, -1.);
        mat.push_with_node_id(node_in, new_pos + 1, 1.);
        mat.push_with_node_id(node_out, new_pos + 1, -1.);
    }
}

impl TimeVaringLinearElement {
    pub fn parse_capacitor(s: &str) -> Option<Self> {
        let (name, node_in, node_out, value) = super::base::general_element_parse(s)?;
        Some(Self {
            name,
            node_in,
            node_out,
            element_type: TimeVaringLinearElementType::Capacitor(value),
        })
    }

    pub fn parse_inductor(s: &str) -> Option<Self> {
        let (name, node_in, node_out, value) = super::base::general_element_parse(s)?;
        Some(Self {
            name,
            node_in,
            node_out,
            element_type: TimeVaringLinearElementType::Inductor(value),
        })
    }
}

impl MatrixSettable for TimeVaringLinearElement {
    fn set_matrix_dc(
        &self,
        mat: &mut crate::matrix::build::MatrixTriplets<f64>,
        v: &mut crate::matrix::build::VecItems<f64>,
    ) {
        match self.element_type {
            TimeVaringLinearElementType::Capacitor(_) => {
                self.set_matrix_dc_capacitor(mat, v);
            }
            TimeVaringLinearElementType::Inductor(_) => {
                self.set_matrix_dc_inductor(mat, v);
            }
        }
    }
}
