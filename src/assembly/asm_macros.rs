use super::{PreProcessor, Statements, Symbol};
use crate::Res;

#[derive(Debug, Clone)]
pub enum Macros<'a> {
    Directive(Function<'a>),
    Substitution(SubMacro<'a>),
}

#[derive(Debug, Clone)]
pub struct SubMacro<'a> {
    pub input: Vec<Symbol<'a>>,
    pub body: Vec<Statements<'a>>,
    pub offset: u32,
}

pub type Function<'a> =
    fn(pp: &mut PreProcessor<'a>, input: Statements<'a>) -> Res<Vec<Statements<'a>>>;

impl<'a> PreProcessor<'a> {
    pub fn standard_directives(&mut self) {
        self.define_std("global", global);
        self.define_std("macro", amacro);
        self.define_std("section", section);
    }
}

pub fn global<'a>(pp: &mut PreProcessor<'a>, input: Statements<'a>) -> Res<Vec<Statements<'a>>> {
    let mut st = Vec::new();
    // println!("{input:#?}");

    let param = if let Statements::Directive { name, params, .. } = input.clone() {
        assert!(name.lexeme().as_ref() == "global", "assertion failed");
        params
    } else {
        todo!()
    };

    let entry = param.first();
    let entry_label = entry.map(|x| x.lexeme());

    pp.entry = entry_label;

    st.push(input);
    Ok(st)
}

pub fn amacro<'a>(pp: &mut PreProcessor<'a>, input: Statements<'a>) -> Res<Vec<Statements<'a>>> {
    let mut st = Vec::new();

    let (params, body, pc) = if let Statements::Directive {
        name,
        params,
        body,
        pc,
        ..
    } = input.clone()
    {
        assert_eq!(name.lexeme().as_ref(), "macro", "assertion failed");
        (params, body, pc)
    } else {
        todo!()
    };

    let name = params.first();
    let params = params[1..].to_vec();

    let sub = SubMacro {
        input: params,
        body,
        offset: pc,
    };

    pp.define_submacro(name.unwrap().lexeme(), sub);

    st.push(input);

    Ok(st)
}

pub fn section<'a>(_pp: &mut PreProcessor<'a>, _input: Statements<'a>) -> Res<Vec<Statements<'a>>> {
    // if input.is_empty() {
    //     return Err(EsiuxErrorKind::Format(Box::new(
    //         "format: .section <name>".to_string(),
    //     )));
    // }

    // let Token { lexeme, .. } = if let Some(Symbol::Ident(label)) = input.first() {
    //     label
    // } else {
    //     return Err(EsiuxErrorKind::DirectiveResolve(
    //         format!(
    //             "Failed to resolve .section: Expected an ident got {:?}",
    //             input[0]
    //         ),
    //         input[0].line(),
    //     ));
    // };

    // pp.section = lexeme.parse::<Section>()?;

    // let resolved = vec![Symbol::Directive(
    //     Token::from("section"),
    //     vec![input[0].clone()],
    // )];

    // Ok(resolved)
    todo!()
}
