use std::fmt::Display;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub enum Expr {
    Literal(String),
    Identifier(String),
    Assign(String, Box<Expr>),
    GlobalDataAddr(String),
    Function(String, Vec<String>, Vec<Expr>)
}

pub fn print_ast(expr: &Expr) {
    use Expr::*;

    match expr {
        Literal(_) | Identifier(_) | GlobalDataAddr(_) | Function(_, _, _) => println!("{}", expr),
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
            Expr::Function(name, _, _) => f.write_fmt(format_args!("Function : {} ", name))
        }
    }
}

peg::parser! {pub grammar parser() for str {
    pub rule file() -> Vec<Expr> = statements() / functions()

   rule statements() -> Vec<Expr>
        = s:(statement()*) { s }

    rule statement() -> Expr
        =  blankline() e:expression()  blankline() { e }

    pub rule functions() -> Vec<Expr> =  s:(function()*) {s}

    rule function() -> Expr = 
    blankline() "def" _ name:identifier() _ "(" _ params:((_ i:identifier() _ {i})) ** "," ")" _ ":" "\n"* stmts:(statement()+) {
       Expr::Function(name, params, stmts) 
    }

    rule expression() -> Expr = assignment() / literal()

   rule assignment() -> Expr = ident:identifier() _ "=" _ expr:expression() {
        Expr::Assign(ident, Box::new(expr))
    }

   rule identifier() -> String
        = quiet!{ n:$(['a'..='z' | 'A'..='Z' | '_']['a'..='z' | 'A'..='Z' | '0'..='9' | '_']*) { n.to_owned() } }
        / expected!("identifier")

    rule literal() -> Expr
        = n:$(['0'..='9']+) { Expr::Literal(n.to_owned()) }
        / "&" i:identifier() { Expr::GlobalDataAddr(i) }

    rule blankline() = quiet!{[' ' | '\t' | '\n']*}
    rule _() =  quiet!{[' ' | '\t']*}


}}

#[cfg(test)]
mod tests {
    use super::*;

    const FUNCTION_NO_PARAMS: &str = "def hello():
    A = 3";
    const FUNCTION_MULTI_PARAM: &str = "def hello(A,B,C):
    A = 3";
    const ASSIGN: &str = "A = 3";

    const MULTI_ASSIGN: &str = "A = 3
B = 4";

    #[test]
    fn test_assignment() -> Result<()> {
        let expr = &parser::file(ASSIGN)?[0];
        matches!(expr, Expr::Assign(_, _));
        Ok(())
    }

    #[test]
    fn test_multi_assign() -> Result<()> {
        let expr = parser::file(MULTI_ASSIGN)?;
        matches!(expr[0], Expr::Assign(_, _));
        matches!(expr[1], Expr::Assign(_, _));
        Ok(())
    }

    #[test]
    fn test_function() -> Result<()> {
        parser::functions(FUNCTION_NO_PARAMS)?;
        parser::functions(FUNCTION_MULTI_PARAM)?;
        Ok(())
    }
}
