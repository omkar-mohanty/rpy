use std::fmt::Display;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub enum Expr {
    Literal(String),
    Identifier(String),
    Assign(String, Box<Expr>),
    GlobalDataAddr(String),
    Function(String, Vec<String>, Vec<Expr>),
    Operation(Box<Expr>, Box<Expr>, BinaryOp)
}

pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div
}

pub fn print_ast(expr: &Expr) {
    use Expr::*;

    match expr {
        Literal(_) | Identifier(_) | GlobalDataAddr(_) | Function(_, _, _) | Operation(_, _, _) => println!("{}", expr),
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
            Expr::Function(name, _, _) => f.write_fmt(format_args!("Function : {} ", name)),
            Expr::Operation(lhs, rhs, _) => f.write_fmt(format_args!("Operation : {} {}", lhs, rhs))
        }
    }
}

peg::parser! {pub grammar parser() for str {
    pub rule file() -> Vec<Expr> = s:statements() { s }

    rule statements() -> Vec<Expr> = s:statement()+ { s }

    rule statement() -> Expr = compound_statement() / simple_statement()

    rule compound_statement() -> Expr = function() / simple_statement()

    rule functions() -> Vec<Expr> = fns:(function()*) {fns}


   rule function() -> Expr =  [' ' | '\t' | '\n']* "def" _ id:name() _ "(" params:((_ i:name() _ {i}) ** ",") ")" _ ":" _ "\n"+  {
        Expr::Identifier(id)
    }

    rule simple_statement() -> Expr = assignment()

    rule assignment() -> Expr = [' ' | '\t' | '\n']* id:name() _ "=" _ expr:expression() "\n"* { Expr::Assign(id, Box::new(expr))}

    rule expression() -> Expr = assignment()  / arithmetic() / literal() 

    rule arithmetic() -> Expr = operand:literal() _ op:op() _ operator:literal() {Expr::Operation(Box::new(operand), Box::new(operator), op)} 

    rule op() -> BinaryOp = add() / sub() / mul() / div() 

    rule add() -> BinaryOp = "+" {BinaryOp::Add}
    rule sub() -> BinaryOp = "-" {BinaryOp::Sub}
    rule div() -> BinaryOp = "/" {BinaryOp::Div}
    rule mul() -> BinaryOp = "*" {BinaryOp::Mul}

    rule name() -> String
        = quiet!{ n:$(['a'..='z' | 'A'..='Z' | '_']['a'..='z' | 'A'..='Z' | '0'..='9' | '_']*) { n.to_owned() } }
        / expected!("identifier")

    rule literal() -> Expr
        = n:number() 
        /  i:name() { Expr::GlobalDataAddr(i) }

    rule number() -> Expr = n:$(['0'..='9']+) {Expr::Literal(n.to_owned())}

    rule _() = quiet!{[' ' | '\t' ]*}
}}

#[cfg(test)]
mod tests {
    use super::*;

    const FUNCTION_NO_PARAMS: &str = "def hello():
    A = 3";
    const FUNCTION_MULTI_PARAM: &str = "def hello(A ,B ,C):
    A = 3";
    const ASSIGN: &str = "A = 3";

    const MULTI_ASSIGN: &str = "A = 3
B = 4";

    const BINARY_OP:&str = "A = 3 + 4
C = 9 + 10
D = A + C";

    #[test]
    fn test_assignment() -> Result<()> {
        let _ = &parser::file(ASSIGN)?[0];
        Ok(())
    }

    #[test]
    fn test_multi_assign() -> Result<()> {
        let _ = parser::file(MULTI_ASSIGN)?;
        Ok(())
    }

    #[test]
    fn test_function() -> Result<()> {
        parser::file(FUNCTION_NO_PARAMS)?;
        parser::file(FUNCTION_MULTI_PARAM)?;
        Ok(())
    }

    #[test]
    fn binary_op() -> Result<()> {
        parser::file(BINARY_OP)?;
        Ok(())
    }
}
