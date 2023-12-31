use super::super::base::{MatrixDcUpdatable, MatrixSettable};
use crate::matrix::build::VecPushWithNodeId;
use crate::netlist::NodeId;

use std::collections::BTreeMap as Map;
use std::sync::{Arc, Mutex};

#[derive(Debug, PartialEq, Eq, Clone)]
pub(super) enum MosfetType {
    Nmos,
    Pmos,
}

#[derive(Debug, Clone, Copy)]
pub struct MosfetModel {
    vth: f64,
    mu: f64,
    lambda: f64,
    cox: f64,
    cj0: f64,
}

#[derive(Debug, Clone)]
pub(super) struct MosfetElementType {
    pub(super) mos_type: MosfetType,
    pub(super) node_d: NodeId,
    pub(super) node_g: NodeId,
    pub(super) node_s: NodeId,
    pub(super) l: f64,
    pub(super) w: f64,
    pub(super) model_id: usize,
}

lazy_static! {
    static ref MOS_MODELS: Arc<Mutex<Map<usize, MosfetModel>>> = Arc::new(Mutex::new(Map::new()));
}

pub fn add_mosfet_model(model_id: usize, model: MosfetModel) {
    let mut mos_models = MOS_MODELS.lock().unwrap();
    mos_models.insert(model_id, model);
}

impl MosfetModel {
    pub fn parse(s: &str) -> (usize, Self) {
        let mut iter = s.split_whitespace();
        let mut read_next = || -> Option<&str> { iter.next() };

        let mut model = Self {
            vth: 0.,
            mu: 0.,
            lambda: 0.,
            cox: 0.,
            cj0: 0.,
        };

        let mut model_id = 0;

        loop {
            let next_token = read_next();
            if next_token.is_none() {
                break;
            }

            let next_token = next_token.unwrap();

            match next_token {
                ".MODEL" => {
                    model_id = read_next().unwrap().parse::<usize>().unwrap();
                }
                "VT" => {
                    model.vth = read_next().unwrap().parse::<f64>().unwrap();
                }
                "MU" => {
                    model.mu = read_next().unwrap().parse::<f64>().unwrap();
                }
                "COX" => {
                    model.cox = read_next().unwrap().parse::<f64>().unwrap();
                }
                "LAMBDA" => {
                    model.lambda = read_next().unwrap().parse::<f64>().unwrap();
                }
                "CJ0" => {
                    model.cj0 = read_next().unwrap().parse::<f64>().unwrap();
                }
                _ => break,
            }
        }

        (model_id, model)
    }
}

enum MosfetMode {
    CutOff,
    Linear,
    Saturation,
}

impl MosfetElementType {
    fn get_model_by_id(model_id: usize) -> MosfetModel {
        MOS_MODELS.lock().unwrap()[&model_id]
    }

    fn get_model(&self) -> MosfetModel {
        Self::get_model_by_id(self.model_id)
    }

    fn get_mode(&self, v_gs: f64, v_ds: f64) -> MosfetMode {
        match self.mos_type {
            MosfetType::Nmos => {
                if v_gs < self.get_model().vth {
                    MosfetMode::CutOff
                } else if v_ds < v_gs - self.get_model().vth {
                    MosfetMode::Linear
                } else {
                    MosfetMode::Saturation
                }
            }
            MosfetType::Pmos => {
                if v_gs > self.get_model().vth {
                    MosfetMode::CutOff
                } else if v_ds > v_gs - self.get_model().vth {
                    MosfetMode::Linear
                } else {
                    MosfetMode::Saturation
                }
            }
        }
    }

    fn get_k(&self) -> f64 {
        let model = self.get_model();
        model.mu * model.cox * self.w / self.l
    }

    fn get_gm(&self, v_gs: f64, v_ds: f64) -> f64 {
        let mos_mode = self.get_mode(v_gs, v_ds);
        let k = self.get_k();
        let model = self.get_model();

        match mos_mode {
            MosfetMode::CutOff => 0.,
            MosfetMode::Linear => k * v_ds,
            MosfetMode::Saturation => k * (v_gs - model.vth) * (1. + model.lambda * v_ds.abs()),
        }
        .abs()
    }

    fn get_gds(&self, v_gs: f64, v_ds: f64) -> f64 {
        let mos_mode = self.get_mode(v_gs, v_ds);
        let k = self.get_k();
        let model = self.get_model();

        match mos_mode {
            MosfetMode::CutOff => 0.,
            MosfetMode::Linear => k * (v_gs - model.vth - v_ds),
            MosfetMode::Saturation => { k * (v_gs - model.vth).powi(2) * model.lambda }.abs(),
        }
        .abs()
    }

    fn get_ids(&self, v_gs: f64, v_ds: f64) -> f64 {
        let mos_mode = self.get_mode(v_gs, v_ds);
        let k = self.get_k();
        let model = self.get_model();

        match mos_mode {
            MosfetMode::CutOff => 0.,
            MosfetMode::Linear => k * (v_gs - model.vth - v_ds * 0.5) * v_ds.abs(),
            MosfetMode::Saturation => match self.mos_type {
                MosfetType::Nmos => {
                    0.5 * k * (v_gs - model.vth).powi(2) * (1. + model.lambda * v_ds.abs())
                }
                MosfetType::Pmos => {
                    -0.5 * k * (v_gs - model.vth).powi(2) * (1. + model.lambda * v_ds.abs())
                }
            },
        }
    }

    fn get_ieq(&self, v_gs: f64, v_ds: f64) -> f64 {
        self.get_ids(v_gs, v_ds) - self.get_gds(v_gs, v_ds) * v_ds - self.get_gm(v_gs, v_ds) * v_gs
    }
}

impl MatrixSettable for MosfetElementType {
    fn set_matrix_dc(
        &self,
        mat: &mut crate::matrix::build::MatrixTriplets<f64>,
        v: &mut crate::matrix::build::VecItems<f64>,
    ) {
        mat.push_with_node_id(self.node_d, self.node_d, 0.);
        mat.push_with_node_id(self.node_d, self.node_s, 0.);
        mat.push_with_node_id(self.node_s, self.node_d, 0.);
        mat.push_with_node_id(self.node_s, self.node_s, 0.);

        v.push_with_node_id(self.node_d, 0.);
        v.push_with_node_id(self.node_s, 0.);
        v.push_with_node_id(self.node_g, 0.);

        mat.push_with_node_id(self.node_d, self.node_g, 0.);
        mat.push_with_node_id(self.node_s, self.node_s, 0.);
        mat.push_with_node_id(self.node_d, self.node_s, 0.);
        mat.push_with_node_id(self.node_s, self.node_g, 0.);
    }
}

impl MatrixDcUpdatable for MosfetElementType {
    fn update_matrix_dc(
        &self,
        mat: &mut sprs::CsMat<f64>,
        v: &mut sprs::CsVec<f64>,
        x: &sprs::CsVec<f64>,
    ) {
        use crate::matrix::ext::{MatExt, VecExt};

        let v_g = x.get_by_node_id(self.node_g);
        let v_d = x.get_by_node_id(self.node_d);
        let v_s = x.get_by_node_id(self.node_s);

        let v_gs = v_g - v_s;
        let v_ds = v_d - v_s;

        {
            // Update gds
            let gds = self.get_gds(v_gs, v_ds);
            mat.add_by_node_id(self.node_d, self.node_d, gds);
            mat.add_by_node_id(self.node_d, self.node_s, -gds);
            mat.add_by_node_id(self.node_s, self.node_d, -gds);
            mat.add_by_node_id(self.node_s, self.node_s, gds);
        }

        {
            // Update ieq
            let ieq = self.get_ieq(v_gs, v_ds);
            v.add_by_node_id(self.node_d, -ieq);
            v.add_by_node_id(self.node_s, ieq);
        }

        {
            // Update gm
            let gm = self.get_gm(v_gs, v_ds);
            mat.add_by_node_id(self.node_d, self.node_g, gm);
            mat.add_by_node_id(self.node_s, self.node_s, gm);
            mat.add_by_node_id(self.node_d, self.node_s, -gm);
            mat.add_by_node_id(self.node_s, self.node_g, -gm);
        }
    }
}
