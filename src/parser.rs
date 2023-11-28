use crate::elements::base::Element;
use crate::elements::time_varing_non_linear::mosfet;
use crate::elements::MosfetModel;

use crate::elements::{BasicElement, TimeVaringLinearElement, TimeVaringNonLinearElement};
use crate::task::Task;

use std::io::BufRead;
use std::{fs::File, path::PathBuf};

pub struct Parser {
    file: PathBuf,
}

pub struct ParsedInfo {
    pub basic_elements: Vec<BasicElement>,
    pub time_varing_linear_elements: Vec<TimeVaringLinearElement>,
    pub time_varing_non_linear_elements: Vec<TimeVaringNonLinearElement>,
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
        let mut basic_elements: Vec<BasicElement> = Vec::new();
        let mut time_varing_linear_elements: Vec<TimeVaringLinearElement> = Vec::new();
        let mut time_varing_non_linear_elements: Vec<TimeVaringNonLinearElement> = Vec::new();

        let mut tasks: Vec<super::task::Task> = Vec::new();

        let lines = std::io::BufReader::new(file).lines();

        for (line, line_no) in lines.zip(1..) {
            let line = line?;
            let trimmed_line = line.trim();

            if trimmed_line.starts_with('*') || trimmed_line.is_empty() {
                continue;
            }

            let first_char = trimmed_line.chars().next().unwrap();

            let mut update_node_info_with_new_element = |element: &dyn Element| {
                for node in element.get_nodes() {
                    node_set.insert(node);
                    max_node_id = max_node_id.max(node);
                }
            };

            match first_char.to_ascii_uppercase() {
                'R' => {
                    let resistor = BasicElement::parse_resistor(trimmed_line).unwrap();
                    update_node_info_with_new_element(&resistor);
                    basic_elements.push(resistor);
                }
                'V' => {
                    let voltage_source = BasicElement::parse_voltage_source(trimmed_line).unwrap();
                    update_node_info_with_new_element(&voltage_source);
                    basic_elements.push(voltage_source);
                }
                'I' => {
                    let current_source = BasicElement::parse_current_source(trimmed_line).unwrap();
                    update_node_info_with_new_element(&current_source);
                    basic_elements.push(current_source);
                }
                'C' => {
                    let capacitor = TimeVaringLinearElement::parse_capacitor(trimmed_line).unwrap();
                    update_node_info_with_new_element(&capacitor);
                    time_varing_linear_elements.push(capacitor);
                }
                'L' => {
                    let inductor = TimeVaringLinearElement::parse_inductor(trimmed_line).unwrap();
                    update_node_info_with_new_element(&inductor);
                    time_varing_linear_elements.push(inductor);
                }
                'M' => {
                    let mosfet = TimeVaringNonLinearElement::parse_mosfet(trimmed_line);
                    update_node_info_with_new_element(&mosfet);
                    time_varing_non_linear_elements.push(mosfet);
                }
                '.' => {
                    let mut words = trimmed_line.split_ascii_whitespace();
                    let directive = words.next().unwrap();
                    match directive.to_ascii_uppercase().as_str() {
                        ".MODEL" => {
                            let (model_id, mosfet_model) = MosfetModel::parse(trimmed_line);
                            mosfet::add_mosfet_model(model_id, mosfet_model);
                        }
                        ".PLOTNV" => {
                            let node_id = words.next().unwrap().parse::<usize>().unwrap();
                            tasks.push(Task::PlotVoltage(node_id));
                        }
                        ".PLOTIB" => {
                            let from = words.next().unwrap().parse::<usize>().unwrap();
                            let to = words.next().unwrap().parse::<usize>().unwrap();
                            tasks.push(Task::PlotCurrent(from, to));
                        }
                        _ => {
                            return Err(format!(
                                "Invalid directive: {}, {}:{}",
                                directive,
                                self.file.display(),
                                line_no
                            )
                            .into());
                        }
                    }
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
            basic_elements,
            time_varing_linear_elements,
            time_varing_non_linear_elements,
            tasks,
            node_num: node_set.len(),
            max_node_id,
        })
    }
}
