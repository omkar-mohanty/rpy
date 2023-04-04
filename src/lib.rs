mod lexer;
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub enum Expr {
    Literal(String),
    Identifier(String),
    Assign(String, Box<Expr>),
    GlobalDataAddr(String),
}

peg::parser! {pub grammar parser() for str {
    pub rule expression() -> Expr
        = assignment() /
          literal()

    rule assignment() -> Expr
        = i:identifier() _ "=" _ e:expression() {Expr::Assign(i, Box::new(e))}



}}
