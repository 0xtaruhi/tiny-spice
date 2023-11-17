use crate::{netlist::NodeId, matrix::build::{MatrixTriplets, VecItems}};

pub enum ComponentType {
    Resistor,
    VoltageSource,
}

pub trait Component {
    fn get_name(&self) -> &str;

    fn get_type(&self) -> ComponentType;

}

pub trait BasicComponent: Component {
    fn get_node_in(&self) -> NodeId;

    fn get_node_out(&self) -> NodeId;
    
    fn get_base_value(&self) -> f64;

    fn set_matrix_dc(&self, mat: &mut MatrixTriplets<f64>, v: &mut VecItems<f64>);
}
