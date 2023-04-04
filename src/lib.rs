pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub enum Expr {
    Literal(String),
    Identifier(String),
    Assign(String, Box<Expr>),
    GlobalDataAddr(String),
}

peg::parser! {pub grammar parser() for str {
    pub rule statements() -> Vec<Expr>
        = s:(statement()*) { s }

    rule statement() -> Expr
        = _ e:expression() _ "\n" { e }

    rule expression() -> Expr
        = assignment() /
          literal()

    rule assignment() -> Expr
        = i:identifier() _ "=" _ e:expression() {Expr::Assign(i, Box::new(e))}

    rule identifier() -> String
        = quiet!{ n:$(['a'..='z' | 'A'..='Z' | '_']['a'..='z' | 'A'..='Z' | '0'..='9' | '_']*) { n.to_owned() } }
        / expected!("identifier")

    rule literal() -> Expr
        = n:$(['0'..='9']+) { Expr::Literal(n.to_owned()) } /
            i:identifier() {Expr::GlobalDataAddr(i)}

     rule _() =  quiet!{[' ' | '\t']*}
}}

#[cfg(test)] 
mod tests {
    use super::*;

    const ASSIGN:&str = r"A = 3";

    const MULTI_ASSIGN:&str = "A = 3";

    #[test]
    fn test_assignment() -> Result<()> {
        let expr = parser::statements(ASSIGN)?;

        assert_eq!(expr.len(), 1);

        Ok(())
    }

    #[test]
    fn test_multi_assign() -> Result<()> {
        let expr = parser::statements(MULTI_ASSIGN)?;

        assert_eq!(expr.len(), 2);

        Ok(())
    }
}
