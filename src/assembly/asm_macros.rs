use std::borrow::Cow;

use crate::{error::EsiuxErrorKind, Res};

use super::PreProcessor;

#[derive(Debug, Clone)]
pub enum Macros<'a> {
    Directive(Function<'a>),
    Substitution(SubMacro<'a>),
}

#[derive(Debug, Clone)]
pub struct SubMacro<'a> {
    pub input: Vec<&'a str>,
    pub body: Vec<&'a str>,
}

pub type Function<'a> =
    fn(pp: &mut PreProcessor<'a>, input: Vec<&'a str>) -> Res<Option<Vec<&'a str>>>;

impl<'a> PreProcessor<'a> {
    pub fn standard_directives(&mut self) {
        self.define_directive("global", global);
        self.define_directive("macro", amacro);
    }
}

pub fn global<'a>(pp: &mut PreProcessor<'a>, input: Vec<&'a str>) -> Res<Option<Vec<&'a str>>> {
    let entry_label = input[0];
    pp.entry = Some(entry_label);
    Ok(None)
}

pub fn amacro<'a>(pp: &mut PreProcessor<'a>, input: Vec<&'a str>) -> Res<Option<Vec<&'a str>>> {
    if input.is_empty() {
        return Err(EsiuxErrorKind::DefineMacro);
    }

    let inputs = input[0].split_whitespace().collect::<Vec<_>>();
    let macro_name = inputs[1];

    let param = inputs.iter().skip(2).map(|x| x.trim()).collect::<Vec<_>>();

    let submacro = SubMacro {
        input: param,
        body: input[1..input.len() - 1].to_vec(),
    };

    pp.define_submacro(macro_name, submacro);

    Ok(None)
}

pub fn section<'a>(pp: &mut PreProcessor<'a>, input: Vec<&'a str>) -> Res<Option<Vec<&'a str>>> {
    if input.is_empty() || input.len() < 2 {
        return Err(EsiuxErrorKind::Format(Box::new(
            "format: .section <name>".to_string(),
        )));
    }

    let section = input[1];

    Ok(None)
}
