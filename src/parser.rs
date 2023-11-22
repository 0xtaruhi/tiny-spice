use crate::elements::base::{LinearElement, TwoPortElement, NonLinearElement};
use crate::elements::capacitor::Capacitor;
use crate::elements::current_source::CurrentSource;
use crate::elements::inductor::Inductor;
use crate::elements::resistor::Resistor;
use crate::elements::voltage_source::VoltageSource;

use std::io::BufRead;
use std::{fs::File, path::PathBuf};

pub struct Parser {
    file: PathBuf,
}

pub struct ParsedInfo {
    pub linear_elements: Vec<Box<dyn LinearElement>>,
    pub non_linear_elements: Vec<Box<dyn NonLinearElement>>,
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
        let mut linear_elements: Vec<Box<dyn LinearElement>> = Vec::new();
        let mut non_linear_elements: Vec<Box<dyn NonLinearElement>> = Vec::new();

        let lines = std::io::BufReader::new(file).lines();

        for (line, line_no) in lines.zip(1..) {
            let line = line?;
            let trimmed_line = line.trim();

            if trimmed_line.starts_with("*") || trimmed_line.is_empty() {
                continue;
            }

            let first_char = trimmed_line.chars().next().unwrap();

            let mut update_node_info_with_two_port_element = |element: &dyn TwoPortElement| {
                node_set.insert(element.get_node_in());
                node_set.insert(element.get_node_out());
                max_node_id = max_node_id.max(element.get_node_in());
                max_node_id = max_node_id.max(element.get_node_out());
            };

            match first_char.to_ascii_uppercase() {
                'R' => {
                    let resistor = Resistor::parse(trimmed_line);
                    update_node_info_with_two_port_element(&resistor);
                    linear_elements.push(Box::new(resistor));
                }
                'V' => {
                    let voltage_source = VoltageSource::parse(trimmed_line);
                    update_node_info_with_two_port_element(&voltage_source);
                    linear_elements.push(Box::new(voltage_source));
                }
                'I' => {
                    let current_source = CurrentSource::parse(trimmed_line);
                    update_node_info_with_two_port_element(&current_source);
                    linear_elements.push(Box::new(current_source));
                }
                'C' => {
                    let capacitor = Capacitor::parse(trimmed_line);
                    update_node_info_with_two_port_element(&capacitor);
                    non_linear_elements.push(Box::new(capacitor));
                }
                'L' => {
                    let inductor = Inductor::parse(trimmed_line);
                    update_node_info_with_two_port_element(&inductor);
                    non_linear_elements.push(Box::new(inductor));
                }
                _ => {
                    return Err(format!(
                        "Invalid element type: {}, {}:{}",
                        first_char,
                        self.file.display(),
                        line_no
                    )
                    .into());
                }
            }
        }

        Ok(ParsedInfo {
            linear_elements: linear_elements,
            non_linear_elements: non_linear_elements,
            tasks: vec![],
            node_num: node_set.len(),
            max_node_id: max_node_id,
        })
    }
}
