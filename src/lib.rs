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

    pub rule function() -> (Expr, Vec<String>, Vec<Expr>) =
    [' ', '\t', '\n']* "def" _ name:identifier() _ "(" params:((_ i:identifier()_{i})** "," )")" _ ":" [' ', '\t', '\n']* stmts:statements() {
        (Expr::Identifier(name), params, stmts)
    }
    
    pub rule statements() -> Vec<Expr>
        = s:(statement()*) { s }

    rule statement() -> Expr
        = _ e:expression() _ "\n" { e }

    rule expression() -> Expr
        = assignment()

     rule assignment() -> Expr
        = i:identifier() _ "=" _ e:expression() {Expr::Assign(i, Box::new(e))}

     rule identifier() -> String
        = quiet!{ n:$(['a'..='z' | 'A'..='Z' | '_']['a'..='z' | 'A'..='Z' | '0'..='9' | '_']*) { n.to_owned() } }
        / expected!("identifier")

    rule literal() -> Expr
        = n:$(['0'..='9']+) { Expr::Literal(n.to_owned()) }
        / "&" i:identifier() { Expr::GlobalDataAddr(i) }

      rule _() =  quiet!{[' ' | '\t']*}
}}

#[cfg(test)]
mod tests {
    use super::*;

    const ASSIGN: &str = "A = 3";

    const MULTI_ASSIGN: &str = "A = 3
    B = A";

    #[test]
    fn test_assignment() -> Result<()> {
        let expr = parser::statements(ASSIGN)?;

        match &expr[0] {
            Expr::Assign(name, val) => {
                assert_eq!("A", name);
            }
            _ => {}
        };

        Ok(())
    }

    #[test]
    fn test_multi_assign() -> Result<()> {
        let expr = parser::statements(MULTI_ASSIGN)?;

        assert_eq!(expr.len(), 2);

        Ok(())
    }
}
