use std::collections::HashMap;

use crate::{format::Section, Res};

use super::Macros;

#[derive(Debug, Default)]
pub struct PreProcessor {
    pub labels: HashMap<String, u32>,
    pub variables: HashMap<String, String>,
    pub macros: HashMap<String, Macros>,
    pub pc: u32,
    pub source: String,
    pub section: Section,
    pub entry: String,
}

impl PreProcessor {
    pub fn new(source: String) -> Self {
        Self {
            source,
            ..Default::default()
        }
    }

    // pub fn define_macro()
    pub fn first_pass(&mut self) -> Res<()> {
        let mut macro_lines = Vec::new();
        let mut in_macro = false;

        for line in self.source.lines() {
            if line.is_empty() || line.starts_with(";") {
                continue;
            }

            let line_components = line
                .split_whitespace()
                .map(|x| x.trim())
                .collect::<Vec<_>>();

            if in_macro {
                macro_lines.push(line);
                match line_components[0] {
                    ".endm" => {
                        in_macro = false;
                    }
                    _ => continue,
                }
            }

            self.section = match line.parse::<Section>() {
                Ok(s) => s,
                Err(_) => self.section,
            };

            if line_components[0].starts_with(".") {
                match line_components[0].trim_start_matches(".") {
                    "macro" => {
                        in_macro = true;
                        macro_lines.push(line);
                    }
                    "global" => {
                        assert!(line_components.len() >= 2);
                        self.entry = line_components[1].to_owned();
                    }
                    _ => continue,
                }
            }

            if self.section == Section::Text && line_components[0].ends_with(":") {
                self.labels.insert(
                    line_components[0].trim_end_matches(":").to_string(),
                    self.pc,
                );
            } else if line_components[0].ends_with(":") {
                todo!("handle variables")
            }
            self.pc += 4;
        }

        Ok(())
    }
}
