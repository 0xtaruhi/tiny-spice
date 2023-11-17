pub type NodeId = usize;
use crate::{
    components::base::BasicComponent,
    matrix::build::{MatrixTriplets, VecItems},
    parser::ParsedInfo,
};
use sprs::{CsMat, CsVec, TriMat};

pub struct Netlist {
    pub node_num: usize, // include ground node
    pub basic_components: Vec<Box<dyn BasicComponent>>,
}

#[derive(Debug)]
pub struct Equation {
    pub mat_a: CsMat<f64>,
    pub vec_b: CsVec<f64>,
}

impl Netlist {
    pub fn new(parsed_info: ParsedInfo) -> Netlist {
        let node_num = parsed_info.node_num;
        let basic_components = parsed_info.basic_components;
        Netlist {
            node_num,
            basic_components,
        }
    }

    pub fn get_equation(&self) -> Equation {
        let mut mat = MatrixTriplets::new(self.node_num - 1);
        let mut v = VecItems::new();

        self.basic_components.iter().for_each(|component| {
            component.set_matrix_dc(&mut mat, &mut v);
        });

        let (rows, cols, vals) = (mat.rows, mat.cols, mat.vals);
        let tri_mat = TriMat::from_triplets((mat.size, mat.size), rows, cols, vals);

        let mat_a = tri_mat.to_csr();
        let vec_b = CsVec::new(mat.size, v.idxs, v.vals);

        Equation { mat_a, vec_b }
    }
}
