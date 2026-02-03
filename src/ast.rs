// This enum defines the operations we support
#[derive(Debug, Clone)]
pub enum Opcode {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
}

// This recursive enum defines the structure of the language
#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    // Identifiers are Strings to support Greek symbols (e.g., "Δ")
    Var(String),
    // Assignment: "x = 5"
    Assign(String, Box<Expr>),
    // Binary Operation: "a + b"
    Op(Box<Expr>, Opcode, Box<Expr>),
    // Unary Negation: "-5"
    Neg(Box<Expr>),
    // Factorial: "5!"
    Factorial(Box<Expr>),
    // Function Call: "sin(x)"
    Call(String, Box<Expr>),
}
