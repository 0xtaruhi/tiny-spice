pub type NodeId = usize;
use crate::{
    elements::base::{BasicElement, TimeVaringLinearElement, TimeVaringNonLinearElement},
    matrix::build::{MatrixTriplets, VecItems},
};
use log::debug;
use sprs::{CsMat, CsVec, TriMat};

pub struct Netlist {
    pub node_num: usize, // include ground node
    pub basic_elements: Vec<Box<dyn BasicElement>>,
    pub time_varing_linear_elements: Vec<Box<dyn TimeVaringLinearElement>>,
    pub time_varing_non_linear_elements: Vec<Box<dyn TimeVaringNonLinearElement>>,
}

#[derive(Debug)]
pub struct Equation {
    pub mat_a: CsMat<f64>,
    pub vec_b: CsVec<f64>,
}
enum EquationType {
    Dc,
    Trans,
}

impl Netlist {
    pub fn get_equation_dc(&self) -> Equation {
        self.get_equation_impl(EquationType::Dc)
    }

    pub fn get_equation_trans(&self) -> Equation {
        self.get_equation_impl(EquationType::Trans)
    }

    fn get_equation_impl(&self, eq_type: EquationType) -> Equation {
        let mut mat = MatrixTriplets::new(self.node_num - 1);
        let mut v = VecItems::new();

        match eq_type {
            EquationType::Dc => {
                self.basic_elements.iter().for_each(|element| {
                    element.set_matrix_dc(&mut mat, &mut v);
                });

                self.time_varing_linear_elements.iter().for_each(|element| {
                    element.set_matrix_dc(&mut mat, &mut v);
                });

                self.time_varing_non_linear_elements
                    .iter()
                    .for_each(|element| {
                        element.set_matrix_dc(&mut mat, &mut v);
                    });
            }
            EquationType::Trans => {
                self.basic_elements.iter().for_each(|element| {
                    element.set_matrix_trans(&mut mat, &mut v);
                });

                self.time_varing_linear_elements.iter().for_each(|element| {
                    element.set_matrix_trans(&mut mat, &mut v);
                });

                self.time_varing_non_linear_elements
                    .iter()
                    .for_each(|element| {
                        element.set_matrix_trans(&mut mat, &mut v);
                    })
            }
        }

        let (rows, cols, vals) = (mat.rows, mat.cols, mat.vals);
        let tri_mat = TriMat::from_triplets((mat.size, mat.size), rows, cols, vals);

        let mat_a = tri_mat.to_csr();

        let mut v = v
            .iter()
            .map(|(i, v)| (*i, *v))
            .collect::<Vec<(usize, f64)>>();
        v.sort_by(|(i1, _), (i2, _)| i1.cmp(i2));

        let vec_b = CsVec::new(
            mat.size,
            v.iter().map(|(i, _)| *i).collect::<Vec<usize>>(),
            v.iter().map(|(_, v)| *v).collect::<Vec<f64>>(),
        );

        debug!("mat:\n{}, vec:\n{}", mat_a.to_dense(), vec_b.to_dense());

        Equation { mat_a, vec_b }
    }
}
