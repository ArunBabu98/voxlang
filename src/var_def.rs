use crate::expr::Expr;
use crate::utils;

#[derive(Debug, PartialEq)]
pub struct VarDef {
    name: String,
    val: Expr,
}

impl VarDef {
    pub fn new(s: &str) -> (&str, Self) {
        let s = &utils::tag("var", s);

        let (s, _) = utils::extract_whitespaces(s);
        let (s, name) = utils::extract_ident(s);
        let (s, _) = utils::extract_whitespaces(s);
        let s = &utils::tag("=", s);
        let (s, _) = utils::extract_whitespaces(s);
        let (s, val) = Expr::new(s);
        (
            s,
            Self {
                name: name.to_string(),
                val,
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expr::{Number, Op};

    #[test]
    fn parse_vardef() {
        assert_eq!(
            VarDef::new("var a = 10/2"),
            (
                "",
                VarDef {
                    name: "a".to_string(),
                    val: Expr {
                        lhs: Number(10),
                        rhs: Number(2),
                        op: Op::Div
                    }
                }
            )
        );
    }
}
