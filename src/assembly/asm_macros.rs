use super::{PreProcessor, Symbol, Token};
use crate::Res;

#[derive(Debug, Clone)]
pub enum Macros<'a> {
    Directive(Function<'a>),
    Substitution(SubMacro<'a>),
}

#[derive(Debug, Clone)]
pub struct SubMacro<'a> {
    pub input: Vec<Token<'a>>,
    pub body: Vec<Symbol<'a>>,
}

pub type Function<'a> =
    fn(pp: &mut PreProcessor<'a>, input: Vec<Symbol<'a>>) -> Res<Vec<Symbol<'a>>>;

impl<'a> PreProcessor<'a> {
    pub fn standard_directives(&mut self) {
        self.define_std("global", global);
        self.define_std("macro", amacro);
        self.define_std("section", section);
    }
}

pub fn global<'a>(_pp: &mut PreProcessor<'a>, _input: Vec<Symbol<'a>>) -> Res<Vec<Symbol<'a>>> {
    // let Token { lexeme, .. } = if let Some(Symbol::Label(token)) = input.first().cloned() {
    //     token
    // } else {
    //     return Err(EsiuxErrorKind::DirectiveResolve(
    //         format!(
    //             "Failed to resolve .global: Expected a label got {:?}",
    //             input[0]
    //         ),
    //         input[0].line(),
    //     ));
    // };

    // pp.entry = Some(lexeme);

    // let mut st = SymbolStream::default();
    // st.push(input[0].clone());
    // // st.push(Symbol::Whitespace(Token::from("\n")));

    // let resolved = Symbol::Directive(Token::from("global"), st.0);

    // Ok(vec![resolved])
    todo!()
}

pub fn amacro<'a>(_pp: &mut PreProcessor<'a>, _input: Vec<Symbol<'a>>) -> Res<Vec<Symbol<'a>>> {
    // if input.is_empty() {
    //     return Err(EsiuxErrorKind::DefineMacro);
    // }

    // let mut st = SymbolStream::default();

    // let mut name = None::<Token<'a>>;
    // let mut params = Vec::new();
    // let mut body = Vec::new();

    // for symbol in input {
    //     // do a global switch for only preprocess true run this if else
    //     st.push(symbol.clone());
    //     match symbol {
    //         Symbol::Ident(t) => {
    //             if name.is_none() {
    //                 name = Some(t);
    //             }
    //         }
    //         Symbol::Input(t) => params.push(t),
    //         _ => body.push(symbol),
    //     }
    // }

    // let sub = SubMacro {
    //     input: params,
    //     body,
    // };

    // let lex = if let Some(Token { lexeme, .. }) = name {
    //     lexeme
    // } else {
    //     todo!();
    // };

    // pp.define_submacro(lex, sub);

    // let subs = Symbol::Macros(Token::from("macro"), st.0);

    // Ok(vec![subs])
    todo!()
}

pub fn section<'a>(_pp: &mut PreProcessor<'a>, _input: Vec<Symbol<'a>>) -> Res<Vec<Symbol<'a>>> {
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
