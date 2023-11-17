use crate::netlist::NodeId;

pub enum ComponentType {
    Resistor,
    VoltageSource,
}

pub trait Component {
    fn get_name(&self) -> &str;

    fn get_type(&self) -> ComponentType;

    fn get_base_value(&self) -> f64;
}

pub trait BasicComponent: Component {
    fn get_node_in(&self) -> NodeId;

    fn get_node_out(&self) -> NodeId;
}
