use crate::netlist::NodeId;

pub enum Task {
    PlotVoltage(NodeId),
    PlotCurrent(NodeId),
}

impl Task {
    #[allow(dead_code)]
    fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Task::PlotVoltage(node_id) => {
                println!("Plotting voltage at node {}", node_id);
            }
            Task::PlotCurrent(node_id) => {
                println!("Plotting current at node {}", node_id);
            }
        }
        Ok(())
    }
}
