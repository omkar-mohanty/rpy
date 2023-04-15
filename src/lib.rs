mod jit;
mod session;

pub use session::Sesssion;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub enum Expr {
    Literal(String),
    Identifier(String),
    Assign(String, Box<Expr>),
    GlobalDataAddr(String),
    Function(String, Vec<String>, Vec<Expr>),
    Operation(Box<Expr>, Box<Expr>, BinaryOp),
    Call(String, Vec<Expr>),
}

pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
}

peg::parser! {pub grammar parser() for str {
    pub rule file() -> Vec<Expr> = s:statements() { s }

    rule statements() -> Vec<Expr> = s:statement()+ { s }

    rule statement() -> Expr = compound_statement() / simple_statement()

    rule compound_statement() -> Expr = function() / call() / simple_statement()

    rule functions() -> Vec<Expr> = fns:(function()*) {fns}


   rule function() -> Expr =  [' ' | '\t' | '\n']* "def" _ id:name() _ "(" params:((_ i:name() _ {i}) ** ",") ")" _ ":" _ "\n"+ stmts:statements() {
        Expr::Function(id, params, stmts)
    }

    rule call() -> Expr =  _ id:name() _ "(" params:((_ i:expression() _ {i}) ** ",") ")" "\n"+ {
        Expr::Call(id, params)
    }

    rule simple_statement() -> Expr = [' ' | '\t' | '\n']*  assign:assignment() {assign}

    rule assignment() -> Expr = id:name() _ "=" _ expr:expression() "\n"* { Expr::Assign(id, Box::new(expr))}

    rule expression() -> Expr = assignment()  / call() / arithmetic() / literal()

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
        / st:string() {Expr::Literal(st)}

    rule number() -> Expr = n:$(['0'..='9']+) {Expr::Literal(n.to_owned())}

    rule string() -> String
    = "\"" s:name() "\"" { s.to_owned() }

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

    const MULTI_ASSIGN: &str = "
A = 3
B = 4";

    const BINARY_OP: &str = "
A = 3 + 4
C = 9 + 10
D = A + B
F = G - 10
";
    const FUNCTION_CALL: &str = " B = FUNC()
FUnc()
";

    const STRING_LITERAL:&str = "A = \"Hello\"";

    const EXTERN_FUNC:&str = "def hello():
            puts(\"Hello\")
        ";

    #[test]
    fn test_string_literal() -> Result<()> {
        let _ = parser::file(STRING_LITERAL)?[0];
        Ok(())
    }

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

    #[test]
    fn func_call() -> Result<()> {
        parser::file(FUNCTION_CALL)?;
        Ok(())
    }

    #[test]
    fn test_extern_func() -> Result<()> {
        parser::file(EXTERN_FUNC)?;
        Ok(())
    }
}
