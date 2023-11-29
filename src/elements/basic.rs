use std::cell::Cell;

use sprs::{CsMat, CsVec};

use super::base::{Element, MatrixSettable, MatrixTransUpdatable};
use crate::matrix::build::VecPushWithNodeId;
use crate::matrix::ext::{MatExt, VecExt};
use crate::netlist::NodeId;

#[derive(Debug, Clone)]
pub enum SourceType {
    AC,
    DC,
}

#[derive(Debug, Clone)]
pub enum ResistorValue {
    #[allow(dead_code)]
    R(f64),
    G(f64),
}

impl ResistorValue {
    pub fn get_g(&self) -> f64 {
        match self {
            ResistorValue::R(val) => 1. / val,
            ResistorValue::G(val) => val.to_owned(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum BasicElementType {
    Resistor(ResistorValue),
    VoltageSource(SourceType, f64, Cell<NodeId>),
    CurrentSource(SourceType, f64),
}

impl BasicElementType {
    pub fn get_extra_node(&self) -> NodeId {
        match self {
            BasicElementType::VoltageSource(_, _, node) => node.get(),
            _ => panic!("This element doesn't have extra node."),
        }
    }

    pub fn set_extra_node(&self, node: NodeId) {
        match self {
            BasicElementType::VoltageSource(_, _, node_cell) => node_cell.set(node),
            _ => panic!("This element doesn't have extra node."),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BasicElement {
    name: String,
    node_in: NodeId,
    node_out: NodeId,
    element_type: BasicElementType,
}

impl BasicElement {
    pub fn new(
        name: String,
        node_in: NodeId,
        node_out: NodeId,
        element_type: BasicElementType,
    ) -> Self {
        Self {
            name,
            node_in,
            node_out,
            element_type,
        }
    }

    #[allow(dead_code)]
    pub fn get_element_type(&self) -> &BasicElementType {
        &self.element_type
    }
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
    pub(super) fn get_node_in(&self) -> NodeId {
        self.node_in
    }

    pub(super) fn get_node_out(&self) -> NodeId {
        self.node_out
    }

    /// Get the base value of the element.
    /// For a resistor, it returns the conductance.
    pub fn get_base_value(&self) -> f64 {
        match &self.element_type {
            BasicElementType::Resistor(value) => value.get_g(),
            BasicElementType::VoltageSource(_, value, ..) => value.clone(),
            BasicElementType::CurrentSource(_, value) => value.clone(),
        }
    }

    pub fn set_base_value<T>(&mut self, value: T)
    where
        T: Into<f64>,
    {
        match &mut self.element_type {
            BasicElementType::Resistor(val) => {
                *val = ResistorValue::G(value.into());
            }
            BasicElementType::VoltageSource(_, val, ..) => {
                *val = value.into();
            }
            BasicElementType::CurrentSource(_, val) => {
                *val = value.into();
            }
        }
    }
}

impl BasicElement {
    fn set_matrix_dc_resistor(
        &self,
        mat: &mut crate::matrix::build::MatrixTriplets<f64>,
        _v: &mut crate::matrix::build::VecItems<f64>,
    ) {
        let g = self.get_base_value();

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

        self.element_type.set_extra_node(new_pos + 1);

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

impl BasicElement {
    fn update_matrix_trans_resistor(
        &self,
        mat: &mut CsMat<f64>,
        _v: &mut sprs::CsVec<f64>,
        _x: &CsVec<f64>,
    ) {
        let g = self.get_base_value();
        let (node_in, node_out) = (self.get_node_in(), self.get_node_out());

        mat.add_by_node_id(node_in, node_out, -g);
        mat.add_by_node_id(node_out, node_in, -g);
        mat.add_by_node_id(node_in, node_in, g);
        mat.add_by_node_id(node_out, node_out, g);
    }

    fn update_matrix_trans_current_source(
        &self,
        _mat: &mut CsMat<f64>,
        v: &mut CsVec<f64>,
        _x: &CsVec<f64>,
    ) {
        let (node_in, node_out) = (self.get_node_in(), self.get_node_out());

        v.add_by_node_id(node_in, -self.get_base_value());
        v.add_by_node_id(node_out, self.get_base_value());
    }

    fn update_matrix_trans_voltage_source(
        &self,
        _mat: &mut CsMat<f64>,
        v: &mut CsVec<f64>,
        _x: &CsVec<f64>,
    ) {
        let extra_pos = self.element_type.get_extra_node();

        v.add_by_node_id(extra_pos, self.get_base_value());
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
                element_type: BasicElementType::VoltageSource(..),
                ..
            } => self.set_matrix_dc_voltage_source(mat, v),
            BasicElement {
                element_type: BasicElementType::CurrentSource(_, _),
                ..
            } => self.set_matrix_dc_current_source(mat, v),
        }
    }
}

impl MatrixTransUpdatable for BasicElement {
    fn update_matrix_trans(
        &self,
        mat: &mut CsMat<f64>,
        v: &mut sprs::CsVec<f64>,
        x: &sprs::CsVec<f64>,
    ) {
        match self {
            BasicElement {
                element_type: BasicElementType::Resistor(_),
                ..
            } => self.update_matrix_trans_resistor(mat, v, x),
            BasicElement {
                element_type: BasicElementType::VoltageSource(..),
                ..
            } => self.update_matrix_trans_voltage_source(mat, v, x),
            BasicElement {
                element_type: BasicElementType::CurrentSource(_, _),
                ..
            } => self.update_matrix_trans_current_source(mat, v, x),
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
            element_type: BasicElementType::Resistor(ResistorValue::G(1. / val)),
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
            element_type: BasicElementType::VoltageSource(source_type, value, Cell::new(0)),
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
