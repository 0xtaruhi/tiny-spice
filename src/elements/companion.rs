use sprs::CsVec;
use std::cell::Cell;

use crate::elements::base::Element;
use crate::elements::basic::ResistorValue;
use crate::matrix::ext::VecExt;
use crate::netlist::Netlist;

use super::base::{MatrixSettable, MatrixTransUpdatable};
use super::basic::{BasicElement, BasicElementType, SourceType};
use super::time_varing_linear::{TimeVaringLinearElement, TimeVaringLinearElementType};
use super::time_varing_non_linear::TimeVaringNonLinearElement;

#[derive(Debug)]
pub enum TimeVaringElement<'a> {
    Linear(&'a TimeVaringLinearElement),
    #[allow(dead_code)]
    NonLinear(&'a TimeVaringNonLinearElement),
}

#[derive(Debug)]
pub struct CompanionModel<'a> {
    element: TimeVaringElement<'a>,
    current: f64,
    companion_elements: Vec<BasicElement>,
}

pub trait InitCompanionElements {
    fn init_companion_elements(&self, netlist: &Netlist) -> Vec<BasicElement>;
}

impl InitCompanionElements for TimeVaringLinearElement {
    fn init_companion_elements(&self, netlist: &Netlist) -> Vec<BasicElement> {
        match self.get_element_type() {
            TimeVaringLinearElementType::Capacitor(_val) => {
                let new_node = netlist.append_new_node();
                vec![
                    BasicElement::new(
                        format!("{}-R", self.get_name()),
                        self.get_node_in(),
                        new_node,
                        BasicElementType::Resistor(ResistorValue::R(1e10)),
                    ),
                    BasicElement::new(
                        format!("{}-V", self.get_name()),
                        new_node,
                        self.get_node_out(),
                        BasicElementType::VoltageSource(SourceType::DC, 0., Cell::new(0)),
                    ),
                ]
            }
            TimeVaringLinearElementType::Inductor(_val) => {
                vec![
                    BasicElement::new(
                        format!("{}-R", self.get_name()),
                        self.get_node_in(),
                        self.get_node_out(),
                        BasicElementType::Resistor(ResistorValue::G(0.)),
                    ),
                    BasicElement::new(
                        format!("{}-I", self.get_name()),
                        self.get_node_in(),
                        self.get_node_out(),
                        BasicElementType::CurrentSource(SourceType::DC, 0.),
                    ),
                ]
            }
        }
    }
}

impl InitCompanionElements for TimeVaringNonLinearElement {
    fn init_companion_elements(&self, _netlist: &Netlist) -> Vec<BasicElement> {
        todo!()
    }
}

impl TimeVaringLinearElement {
    fn is_capacitor(&self) -> bool {
        matches!(
            self.get_element_type(),
            TimeVaringLinearElementType::Capacitor(_)
        )
    }

    fn is_inductor(&self) -> bool {
        matches!(
            self.get_element_type(),
            TimeVaringLinearElementType::Inductor(_)
        )
    }
}

impl<'a> CompanionModel<'a> {
    pub fn new_from_linear(element: &'a TimeVaringLinearElement, netlist: &Netlist) -> Self {
        let companion_elements = element.init_companion_elements(netlist);
        Self {
            element: TimeVaringElement::Linear(element),
            current: 0.,
            companion_elements,
        }
    }

    fn is_capacitor(&self) -> bool {
        match self.element {
            TimeVaringElement::Linear(element) => element.is_capacitor(),
            _ => false,
        }
    }

    fn is_inductor(&self) -> bool {
        match self.element {
            TimeVaringElement::Linear(element) => element.is_inductor(),
            _ => false,
        }
    }

    fn get_time_varing_element(&self) -> &TimeVaringElement {
        &self.element
    }

    fn get_companion_resistor_mut(&mut self) -> &mut BasicElement {
        assert!(self.is_capacitor() || self.is_inductor());
        match self.element {
            TimeVaringElement::Linear(element) => match element.get_element_type() {
                TimeVaringLinearElementType::Capacitor(_) => &mut self.companion_elements[0],
                TimeVaringLinearElementType::Inductor(_) => &mut self.companion_elements[0],
            },
            _ => panic!("Not a linear element"),
        }
    }

    fn get_companion_resistor(&self) -> &BasicElement {
        assert!(self.is_capacitor() || self.is_inductor());
        match self.element {
            TimeVaringElement::Linear(element) => match element.get_element_type() {
                TimeVaringLinearElementType::Capacitor(_) => &self.companion_elements[0],
                TimeVaringLinearElementType::Inductor(_) => &self.companion_elements[0],
            },
            _ => panic!("Not a linear element"),
        }
    }

    fn get_base_value(&self) -> f64 {
        match self.get_time_varing_element() {
            TimeVaringElement::Linear(element) => element.get_base_value(),
            _ => todo!(),
        }
    }

    fn get_companion_current_source_mut(&mut self) -> &mut BasicElement {
        assert!(self.is_capacitor() || self.is_inductor());

        match self.element {
            TimeVaringElement::Linear(element) => match element.get_element_type() {
                TimeVaringLinearElementType::Inductor(_) => &mut self.companion_elements[1],
                TimeVaringLinearElementType::Capacitor(_) => {
                    panic!("The companion model of an capacitor does not have a current source")
                }
            },
            _ => panic!("Not a linear element"),
        }
    }

    fn get_companion_current_source(&self) -> &BasicElement {
        assert!(self.is_capacitor() || self.is_inductor());

        match self.element {
            TimeVaringElement::Linear(element) => match element.get_element_type() {
                TimeVaringLinearElementType::Inductor(_) => &self.companion_elements[1],
                TimeVaringLinearElementType::Capacitor(_) => {
                    panic!("The companion model of an capacitor does not have a current source")
                }
            },
            _ => panic!("Not a linear element"),
        }
    }

    fn get_companion_voltage_source_mut(&mut self) -> &mut BasicElement {
        assert!(self.is_capacitor() || self.is_inductor());

        match self.element {
            TimeVaringElement::Linear(element) => match element.get_element_type() {
                TimeVaringLinearElementType::Inductor(_) => {
                    panic!("The companion model of a inductor does not have a voltage source")
                }
                TimeVaringLinearElementType::Capacitor(_) => &mut self.companion_elements[1],
            },
            _ => panic!("Not a linear element"),
        }
    }

    fn get_companion_voltage_source(&self) -> &BasicElement {
        assert!(self.is_capacitor() || self.is_inductor());

        match self.element {
            TimeVaringElement::Linear(element) => match element.get_element_type() {
                TimeVaringLinearElementType::Inductor(_) => {
                    panic!("The companion model of a inductor does not have a voltage source")
                }
                TimeVaringLinearElementType::Capacitor(_) => &self.companion_elements[1],
            },
            _ => panic!("Not a linear element"),
        }
    }

    pub fn update_companion_elements(&mut self, x: &CsVec<f64>, delta_t: f64) {
        let base_value = self.get_base_value();
        let current = self.current;

        match self.get_time_varing_element() {
            TimeVaringElement::Linear(element) => match element.get_element_type() {
                TimeVaringLinearElementType::Capacitor(_val) => {
                    let v_diff = x.get_by_node_id(element.get_node_in())
                        - x.get_by_node_id(element.get_node_out());

                    let resistor = self.get_companion_resistor_mut();
                    resistor.set_resistor_value(ResistorValue::R(delta_t / (2. * base_value)));
                    let voltage_source = self.get_companion_voltage_source_mut();
                    voltage_source.set_base_value(v_diff + delta_t * current / (2. * base_value));
                }
                TimeVaringLinearElementType::Inductor(_val) => {
                    let v_diff = x.get_by_node_id(element.get_node_in())
                        - x.get_by_node_id(element.get_node_out());

                    let resistor = self.get_companion_resistor_mut();
                    resistor.set_resistor_value(ResistorValue::G(delta_t / (2. * base_value)));
                    let current_source = self.get_companion_current_source_mut();
                    current_source.set_base_value(current + delta_t * v_diff / (2. * base_value));
                }
            },
            _ => todo!(),
        }
    }

    pub fn update_current(&mut self, x: &CsVec<f64>) {
        let (node_in, node_out) = match self.get_time_varing_element() {
            TimeVaringElement::Linear(element) => (element.get_node_in(), element.get_node_out()),
            _ => todo!(),
        };

        let v_diff = x.get_by_node_id(node_in) - x.get_by_node_id(node_out);
        let new_current;
        match self.get_time_varing_element() {
            TimeVaringElement::Linear(element) => match element.get_element_type() {
                TimeVaringLinearElementType::Capacitor(_val) => {
                    let v_r = v_diff - self.get_companion_voltage_source().get_base_value();
                    let resistor = self.get_companion_resistor();

                    new_current = resistor.get_base_value() * v_r;
                }
                TimeVaringLinearElementType::Inductor(_val) => {
                    let resistor = self.get_companion_resistor();
                    let current_source = self.get_companion_current_source();

                    new_current =
                        resistor.get_base_value() * v_diff + current_source.get_base_value();
                }
            },
            _ => todo!(),
        }
        self.current = new_current;
    }
}

impl<'a> MatrixTransUpdatable for CompanionModel<'a> {
    fn update_matrix_trans(
        &self,
        mat: &mut sprs::CsMat<f64>,
        v: &mut sprs::CsVec<f64>,
        x: &sprs::CsVec<f64>,
    ) {
        for element in &self.companion_elements {
            element.update_matrix_trans(mat, v, x);
        }
    }
}

impl<'a> MatrixSettable for CompanionModel<'a> {
    fn set_matrix_dc(
        &self,
        mat: &mut crate::matrix::build::MatrixTriplets<f64>,
        v: &mut crate::matrix::build::VecItems<f64>,
    ) {
        for element in &self.companion_elements {
            element.set_matrix_dc(mat, v);
        }
    }

    fn set_matrix_trans(
        &self,
        mat: &mut crate::matrix::build::MatrixTriplets<f64>,
        v: &mut crate::matrix::build::VecItems<f64>,
    ) {
        for element in &self.companion_elements {
            element.set_matrix_trans(mat, v);
        }
    }
}
