use crate::netlist::NodeId;

use super::base::{Element, MatrixDcUpdatable, MatrixSettable};

pub mod mosfet;
use mosfet::{MosfetElementType, MosfetType};

#[derive(Debug, Clone)]
enum TimeVaringNonLinearElementType {
    Mosfet(mosfet::MosfetElementType),
}

#[derive(Debug, Clone)]
pub struct TimeVaringNonLinearElement {
    name: String,
    element_type: TimeVaringNonLinearElementType,
}

impl TimeVaringNonLinearElement {
    pub fn parse_mosfet(s: &str) -> Self {
        let mut iter = s.split_whitespace();
        let name = iter.next().unwrap().to_string();
        let node_d = iter.next().unwrap().parse::<NodeId>().unwrap();
        let node_g = iter.next().unwrap().parse::<NodeId>().unwrap();
        let node_s = iter.next().unwrap().parse::<NodeId>().unwrap();
        let mos_type = match iter.next().unwrap() {
            "N" | "n" => MosfetType::Nmos,
            "P" | "p" => MosfetType::Pmos,
            _ => panic!("Invalid mosfet type"),
        };
        let w = iter.next().unwrap().parse::<f64>().unwrap();
        let l = iter.next().unwrap().parse::<f64>().unwrap();

        let model_id = iter.next().unwrap().parse::<usize>().unwrap();

        Self {
            name,
            element_type: TimeVaringNonLinearElementType::Mosfet(MosfetElementType {
                mos_type,
                node_d,
                node_g,
                node_s,
                l,
                w,
                model_id,
            }),
        }
    }
}

impl Element for TimeVaringNonLinearElement {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_nodes(&self) -> Vec<NodeId> {
        match self.element_type {
            TimeVaringNonLinearElementType::Mosfet(MosfetElementType {
                node_d,
                node_g,
                node_s,
                ..
            }) => vec![node_d, node_g, node_s],
        }
    }
}

impl MatrixDcUpdatable for TimeVaringNonLinearElement {
    fn update_matrix_dc(
        &self,
        mat: &mut sprs::CsMat<f64>,
        v: &mut sprs::CsVec<f64>,
        x: &sprs::CsVec<f64>,
    ) {
        match self.element_type {
            TimeVaringNonLinearElementType::Mosfet(ref mosfet) => {
                mosfet.update_matrix_dc(mat, v, x);
            }
        }
    }
}

impl MatrixSettable for TimeVaringNonLinearElement {
    fn set_matrix_dc(
        &self,
        mat: &mut crate::matrix::build::MatrixTriplets<f64>,
        v: &mut crate::matrix::build::VecItems<f64>,
    ) {
        match self.element_type {
            TimeVaringNonLinearElementType::Mosfet(ref mosfet) => {
                mosfet.set_matrix_dc(mat, v);
            }
        }
    }
}
