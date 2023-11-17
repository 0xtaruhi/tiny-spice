pub type NodeId = usize;
use crate::{
    elements::base::{LinearElement, NonLineaerTwoPortElement},
    matrix::build::{MatrixTriplets, VecItems},
    parser::ParsedInfo,
};
use sprs::{CsMat, CsVec, TriMat};

pub struct Netlist {
    pub node_num: usize, // include ground node
    pub linear_elements: Vec<Box<dyn LinearElement>>,
    pub nonlinear_two_port_elements: Vec<Box<dyn NonLineaerTwoPortElement>>,
}

#[derive(Debug)]
pub struct Equation {
    pub mat_a: CsMat<f64>,
    pub vec_b: CsVec<f64>,
}

impl Netlist {
    pub fn new(parsed_info: ParsedInfo) -> Netlist {
        let node_num = parsed_info.node_num;
        let linear_elements = parsed_info.linear_elements;
        let nonlinear_two_port_elements = parsed_info.non_linear_two_port_elements;
        Netlist {
            node_num,
            linear_elements,
            nonlinear_two_port_elements,
        }
    }

    pub fn get_equation(&self) -> Equation {
        let mut mat = MatrixTriplets::new(self.node_num - 1);
        let mut v = VecItems::new();

        self.linear_elements.iter().for_each(|element| {
            element.set_matrix_dc(&mut mat, &mut v);
        });

        self.nonlinear_two_port_elements.iter().for_each(|element| {
            element.set_matrix_dc(&mut mat, &mut v);
        });

        let (rows, cols, vals) = (mat.rows, mat.cols, mat.vals);
        let tri_mat = TriMat::from_triplets((mat.size, mat.size), rows, cols, vals);

        let mat_a = tri_mat.to_csr();

        v.sort_by_key(|(i, _)| *i);

        let vec_b = CsVec::new(
            mat.size,
            v.iter().map(|(i, _)| *i).collect::<Vec<usize>>(),
            v.iter().map(|(_, v)| *v).collect::<Vec<f64>>(),
        );

        Equation { mat_a, vec_b }
    }
}
