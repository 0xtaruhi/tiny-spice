use super::base::{
    Element, ElementType, MatrixSettable, MatrixUpdatable, NonLinearElement, TwoPortElement,
};
use crate::netlist::NodeId;

#[derive(Debug)]
pub struct Inductor {
    name: String,
    node_in: NodeId,
    node_out: NodeId,
    value: f64,
}

impl Inductor {
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

impl Element for Inductor {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_type(&self) -> ElementType {
        ElementType::Inductor
    }

    fn get_nodes(&self) -> Vec<NodeId> {
        vec![self.node_in, self.node_out]
    }
}

impl TwoPortElement for Inductor {
    fn get_node_in(&self) -> NodeId {
        self.node_in
    }

    fn get_node_out(&self) -> NodeId {
        self.node_out
    }
    fn get_base_value(&self) -> f64 {
        self.value
    }
}

impl MatrixSettable for Inductor {
    fn set_matrix_dc(
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

impl MatrixUpdatable for Inductor {
    fn update_matrix_dc(
        &self,
        _mat: &mut sprs::CsMat<f64>,
        _v: &mut sprs::CsVec<f64>,
        _x: &sprs::CsVec<f64>,
    ) {
    }
}

impl NonLinearElement for Inductor {}
