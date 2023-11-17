use crate::components::base::BasicComponent;
use crate::components::current_source::CurrentSource;
use crate::components::resistor::Resistor;
use crate::components::voltage_source::VoltageSource;

use std::io::BufRead;
use std::{fs::File, path::PathBuf};

pub struct Parser {
    file: PathBuf,
}

pub struct ParsedInfo {
    pub basic_components: Vec<Box<dyn BasicComponent>>,
    pub tasks: Vec<super::task::Task>,
    pub node_num: usize,
    pub max_node_id: usize,
}

impl Parser {
    pub fn new(file: PathBuf) -> Self {
        Self { file }
    }

    pub fn parse(&self) -> Result<ParsedInfo, Box<dyn std::error::Error>> {
        fn open_file(file_path: &PathBuf) -> Result<File, std::io::Error> {
            File::open(file_path)
        }

        use std::collections::HashSet;
        let mut node_set: HashSet<usize> = HashSet::new();
        let mut max_node_id = 0;

        let file = open_file(&self.file)?;
        let mut basic_components: Vec<Box<dyn BasicComponent>> = Vec::new();

        let lines = std::io::BufReader::new(file).lines();

        for (line, line_no) in lines.zip(1..) {
            let line = line?;
            let trimmed_line = line.trim();

            if trimmed_line.starts_with("*") || trimmed_line.is_empty() {
                continue;
            }

            let first_char = trimmed_line.chars().next().unwrap();

            let mut update_node_info_with_basic_component = |component: &dyn BasicComponent| {
                node_set.insert(component.get_node_in());
                node_set.insert(component.get_node_out());
                max_node_id = max_node_id.max(component.get_node_in());
                max_node_id = max_node_id.max(component.get_node_out());
            };

            match first_char.to_ascii_uppercase() {
                'R' => {
                    let resistor = Resistor::parse(trimmed_line);
                    update_node_info_with_basic_component(&resistor);
                    basic_components.push(Box::new(resistor));
                }
                'V' => {
                    let voltage_source = VoltageSource::parse(trimmed_line);
                    update_node_info_with_basic_component(&voltage_source);
                    basic_components.push(Box::new(voltage_source));
                }
                'I' => {
                    let current_source = CurrentSource::parse(trimmed_line);
                    update_node_info_with_basic_component(&current_source);
                    basic_components.push(Box::new(current_source));
                }
                _ => {
                    return Err(format!(
                        "Invalid component type: {}, {}:{}",
                        first_char,
                        self.file.display(),
                        line_no
                    )
                    .into());
                }
            }
        }

        Ok(ParsedInfo {
            basic_components: basic_components,
            tasks: vec![],
            node_num: node_set.len(),
            max_node_id: max_node_id,
        })
    }
}
