pub enum Task {
    Plot(),
}

impl Task {
    fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Task::Plot() => {
                println!("Plotting!");
            }
        }
        Ok(())
    }
}
