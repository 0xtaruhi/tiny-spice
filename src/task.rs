use sprs::CsVec;

use crate::{
    netlist::NodeId,
    plot::{plot, PlotInfo},
};

#[derive(Debug)]
pub enum Task {
    PlotVoltage(NodeId),
    PlotCurrent(NodeId, NodeId),
}

#[derive(Debug)]
pub enum TaskResult {
    Voltage {
        node_id: NodeId,
        values: Vec<f64>,
    },
    Current {
        from: NodeId,
        to: NodeId,
        values: Vec<f64>,
    },
}

impl TaskResult {
    pub fn append_value(&mut self, val: f64) {
        match self {
            TaskResult::Voltage { values, .. } => values.push(val),
            TaskResult::Current { values, .. } => values.push(val),
        }
    }

    pub fn update(&mut self, x: &CsVec<f64>) {
        use crate::matrix::ext::VecExt;
        match self {
            TaskResult::Voltage { node_id, .. } => {
                let val = x.get_by_node_id(*node_id);
                self.append_value(val);
            }
            TaskResult::Current { .. } => {
                todo!()
            }
        }
    }

    pub fn run(&self, time_stamps: &[f64]) {
        match self {
            TaskResult::Voltage { node_id, values } => {
                let file_name = format!("voltage_node_{}.png", node_id);
                let caption = format!("Voltage at node {}", node_id);
                let plot_info =
                    PlotInfo::new(&time_stamps, &values, "Time / s", "Voltage / V", &caption);
                plot(plot_info, &file_name);
            }
            TaskResult::Current { .. } => {
                todo!()
            }
        }
    }
}

impl TaskResult {
    pub fn new(task: &Task) -> Self {
        match task {
            Task::PlotVoltage(node_id) => TaskResult::Voltage {
                node_id: node_id.to_owned(),
                values: Vec::new(),
            },
            Task::PlotCurrent(from, to) => TaskResult::Current {
                from: from.to_owned(),
                to: to.to_owned(),
                values: Vec::new(),
            },
        }
    }
}
