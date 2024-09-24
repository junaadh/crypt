use super::Symbol;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum Statements<'a> {
    DPI {
        instruction: Symbol<'a>,
        op1: Symbol<'a>,
        op2: Symbol<'a>,
        op3: Option<Symbol<'a>>,
    },
    LSI {
        instruction: Symbol<'a>,
        op1: Symbol<'a>,
        obracket: bool,
        op2: Symbol<'a>,
        cbracket: bool,
        op3: Symbol<'a>,
    },
    BRI {
        instruction: Symbol<'a>,
        label: Symbol<'a>,
    },
    SCI {
        instruction: Symbol<'a>,
        vector: Symbol<'a>,
    },
    Directive {
        name: Symbol<'a>,
        params: &'a [Symbol<'a>],
        body: &'a [Statements<'a>],
    },
    Substitution {
        name: Symbol<'a>,
        values: Symbol<'a>,
    },
    Label {
        name: Symbol<'a>,
    },
    Comment {
        name: Symbol<'a>,
    },
}
