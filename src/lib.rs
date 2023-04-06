use std::fmt::Display;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub enum Expr {
    Literal(String),
    Identifier(String),
    Assign(String, Box<Expr>),
    GlobalDataAddr(String),
}

pub fn print_ast(expr: &Expr) {
    use Expr::*;

    match expr {
        Literal(_) | Identifier(_) | GlobalDataAddr(_) => println!("{}", expr),
        Assign(var, val) => {
            println!("{} {}", var, val)
        }
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Literal(lit) => f.write_fmt(format_args!("Lit : {}", lit)),
            Expr::Assign(var, val) => {
                f.write_fmt(format_args!("Assign : {}\t", var))?;
                val.fmt(f)
            }
            Expr::Identifier(id) => f.write_fmt(format_args!("Ident : {}", id)),
            Expr::GlobalDataAddr(addr) => f.write_fmt(format_args!("GlobalAddr : {}", addr)),
        }
    }
}

peg::parser! {pub grammar parser() for str {
    pub rule file() -> Vec<Expr> = statements() 

     rule statements() -> Vec<Expr>
        = s:(statement()*) { s }

    rule statement() -> Expr
        = _ e:expression() _ "\n" { e }

    rule expression() -> Expr = assignment() / literal()

    pub rule assignment() -> Expr = _ ident:identifier() _ "=" _ expr:expression() {
        Expr::Assign(ident, Box::new(expr))    
    }

    rule literal() -> Expr
        = n:$(['0'..='9']+) { Expr::Literal(n.to_owned()) }
        / i:identifier() { Expr::GlobalDataAddr(i) }

    pub rule identifier() -> String
        = quiet!{ n:$(['a'..='z' | 'A'..='Z' ]*) { n.to_owned() } }
        / expected!("identifier")

    rule endl() = quiet!{"\n"*}

    rule blankline() = quiet!{[ ' ' | '\t' | '\n' ]*}

    rule _() =  quiet!{[' ' | '\t']*}
}}

#[cfg(test)]
mod tests {
    use super::*;

    const IDENT:&str = "A";

    const FUNCTION:&str = "def hello():";

    const ASSIGN: &str = "A = 3";

    const MULTI_ASSIGN: &str = "A = 3
    B = A";

    #[test]
    fn test_ident() -> Result<()> {
        let expr = parser::identifier(IDENT)?;
        assert_eq!(expr.as_str(), "A");
        Ok(())
    }

    #[test]
    fn test_assignment() -> Result<()> {
        let expr = parser::assignment(ASSIGN)?;

        Ok(())
    }

    #[test]
    fn test_multi_assign() -> Result<()> {
        let expr = parser::file(MULTI_ASSIGN)?;

        Ok(())
    }

    #[test]
    fn test_function() -> Result<()> {
        let expr = parser::file(FUNCTION)?;

        Ok(())
    }
}
