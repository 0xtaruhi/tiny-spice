pub type NodeId = usize;
use crate::{
    matrix::build::{MatrixTriplets, VecItems},
    parser::ParsedInfo,
};
use sprs::{CsMat, CsVec, TriMat};

#[derive(Debug)]
pub struct Netlist {
    pub mat_a: CsMat<f64>,
    pub vec_b: CsVec<f64>,
    pub node_num: usize, // include ground node
}

impl Netlist {
    pub fn new(parsed_info: &ParsedInfo) -> Result<Netlist, Box<dyn std::error::Error>> {
        let mut mat = MatrixTriplets::new(parsed_info.max_node_id);
        let mut v = VecItems::new();

        parsed_info.components.iter().for_each(|component| {
            component.update(&mut mat, &mut v);
        });

        let (rows, cols, vals) = (mat.rows, mat.cols, mat.vals);
        let tri_mat = TriMat::from_triplets((mat.size, mat.size), rows, cols, vals);

        let mat_a = tri_mat.to_csc();
        let vec_b = CsVec::new(mat.size, v.idxs, v.vals);

        Ok(Netlist { mat_a, vec_b, node_num: parsed_info.max_node_id + 1 })
    }
}
