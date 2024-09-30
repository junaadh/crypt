use std::{collections::HashMap, fs, io::Read, process};

use crate::{format::EsiuxBin, Res};

use super::PreProcessor;

#[derive(Debug, Default)]
pub struct Assembler {
    pub labels: HashMap<String, u32>,
    pub source: String,
}

impl Assembler {
    pub fn new(file: &str) -> Res<Self> {
        let mut file = fs::File::open(file)?;
        let mut s = String::new();

        file.read_to_string(&mut s)?;

        Ok(Self {
            labels: HashMap::new(),
            source: s,
        })
    }

    pub fn collect_labels(&mut self) {
        let mut offset = 0;

        for line in self.source.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            if line.ends_with(":") {
                let label = line.trim_end_matches(":").to_string();
                self.labels.insert(label, offset);
            } else {
                offset += 4;
            }
        }
    }

    pub fn resolve_labels(&self, branch: &str) -> String {
        let mut parts = branch.split_whitespace().collect::<Vec<_>>();
        let offset = *self.labels.get(parts[1]).unwrap();

        let fmt = format!("#{offset}");
        parts[1] = fmt.as_str();

        parts.join(" ")
    }

    pub fn assemble(&mut self, preprocess: bool) -> Res<EsiuxBin> {
        let mut pp = PreProcessor::new(&self.source);
        pp.handle()?;

        if preprocess {
            for stmt in pp.intern_buf {
                println!("{stmt}");
            }
            process::exit(0);
        }

        todo!()
    }
}
