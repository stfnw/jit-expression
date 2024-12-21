use std::error;
use std::ffi;
use std::fmt;

/*******************************************************************************
 * Main. ***********************************************************************
 ******************************************************************************/

fn main() -> Result<(), Box<dyn error::Error>> {
    // Read expression from commandline.
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        panic!(
            "Usage: {} 'expression'",
            std::env::current_exe().unwrap().display()
        );
    }

    let tokens = lex(&args[1])?;
    let ast = parse(tokens)?;
    eval_and_jit(ast);

    Ok(())
}

/// Evaluate the given expression syntax tree; once by jitting it down to
/// assembly, once by walking the AST and interpreting it.
/// Finally, both results are asserted to be equal with each other.
fn eval_and_jit(ast: Ast) {
    if let Some(res_interpreter) = eval(&ast) {
        // Only perform further actions (including JIT) if there were no
        // arithmetic errors during interpretation.
        println!("AST: {}", ast);
        println!("res_interpreter: {}", res_interpreter);
        let jitcode = jit(ast);
        println!("JIT CODE:");
        println!("{}", jitcode);
        let res_jit = run_jit(&jitcode);
        println!("res_jit:         {}", res_jit);

        assert_eq!(res_interpreter, res_jit);
    } else {
        println!("Arithmetic error occurred (bounds over/underflow, divide by zero, etc.)!");
    }
}

/// Our calculator works with signed 64 bit numbers only.
type Num = i64;

/*******************************************************************************
 * Lexing. *********************************************************************
 ******************************************************************************/

#[derive(Debug)]
#[allow(dead_code)]
enum LexError {
    UnexpectedChar(char),
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl error::Error for LexError {}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Token {
    Num(Num),
    Plus,
    Minus,
    Mul,
    Div,
    LParen,
    RParen,
}

/// Tokenize an expression string.
fn lex(expr: &str) -> Result<Vec<Token>, LexError> {
    let mut tokens = Vec::new();
    let n = expr.chars().count();

    let mut i = 0;
    while i < n {
        match expr.chars().nth(i).unwrap() {
            ' ' => {
                // Skip whitespace without emitting a token.
                while i < n && expr.chars().nth(i).unwrap() == ' ' {
                    i += 1;
                }
            }

            c if c.is_ascii_digit() => {
                let mut val: Num = 0;
                while let Some(d) = expr.chars().nth(i).unwrap().to_digit(10) {
                    val *= 10;
                    val += d as Num;
                    i += 1;
                }
                tokens.push(Token::Num(val));
            }

            '+' => {
                tokens.push(Token::Plus);
                i += 1;
            }
            '-' => {
                tokens.push(Token::Minus);
                i += 1;
            }
            '*' => {
                tokens.push(Token::Mul);
                i += 1;
            }
            '/' => {
                tokens.push(Token::Div);
                i += 1;
            }
            '(' => {
                tokens.push(Token::LParen);
                i += 1;
            }
            ')' => {
                tokens.push(Token::RParen);
                i += 1;
            }

            c => return Err(LexError::UnexpectedChar(c)),
        }
    }

    Ok(tokens)
}

/*******************************************************************************
 * Parsing. ********************************************************************
 ******************************************************************************/

/// (Abstract) syntax tree of arithmetic expressions.
#[derive(Debug)]
enum Ast {
    Num(Num),
    BinOp(BinOp, Box<Ast>, Box<Ast>),
}

impl fmt::Display for Ast {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Ast::Num(n) => write!(f, "{}", n),
            Ast::BinOp(op, a, b) => write!(f, "({} {} {})", a, op, b),
        }
    }
}

#[derive(Clone, Debug)]
enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

impl fmt::Display for BinOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinOp::Add => write!(f, "+"),
            BinOp::Sub => write!(f, "-"),
            BinOp::Mul => write!(f, "*"),
            BinOp::Div => write!(f, "/"),
        }
    }
}

/// AST node without references to other nodes (that is, this struct contains
/// only the valuess associated with each node). This is used during
/// interpretation/evaluation of the expression through a stack machine after
/// post-order traversal and converting the expression to reverse polish
/// notation.
#[derive(Debug)]
enum AstVal {
    Num(Num),
    BinOp(BinOp),
}

#[derive(Debug)]
#[allow(dead_code)]
enum ParseError {
    UnconsumdedInput(Vec<Token>),
    UnexpectedEOF,
    NoNumAfterUnaryMinus,
    UnbalancedParanthesis,
    UnexpectedToken(Token),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl error::Error for ParseError {}

/// Parse an arithmetic expression into its (abstract) syntax tree using a
/// recurisve descent parser. All intermediate parse functions return a result
/// of a tuple of the currently parsed AST, and a slice of the rest of the yet
/// unparsed token stream. This parser supports operator precedence and grouping
/// in theh following order (from strong to weak):
///   ()    (parenthesis)
///   -     (unary minus)
///   + -   (addition/subtraction)
///   * /   (multiplication/division)
fn parse(tokens: Vec<Token>) -> Result<Ast, ParseError> {
    let (ast, rest) = parse_expr(&tokens[..])?;
    if !rest.is_empty() {
        Err(ParseError::UnconsumdedInput(rest.to_vec()))
    } else {
        Ok(ast)
    }
}

/// Parse additions / subtractions.
fn parse_expr(tokens: &[Token]) -> Result<(Ast, &[Token]), ParseError> {
    let (mut op1, mut tokens) = parse_term(tokens)?;

    while tokens.first() == Some(&Token::Plus) || tokens.first() == Some(&Token::Minus) {
        let binop = if tokens.first() == Some(&Token::Plus) {
            BinOp::Add
        } else {
            BinOp::Sub
        };
        let tmp = parse_term(&tokens[1..])?;
        let op2 = tmp.0;
        tokens = tmp.1;
        op1 = Ast::BinOp(binop, Box::new(op1), Box::new(op2));
    }

    Ok((op1, tokens))
}

/// Parse multiplications / division.
fn parse_term(tokens: &[Token]) -> Result<(Ast, &[Token]), ParseError> {
    let (mut op1, mut tokens) = parse_factor(tokens)?;

    while tokens.first() == Some(&Token::Mul) || tokens.first() == Some(&Token::Div) {
        let binop = if tokens.first() == Some(&Token::Mul) {
            BinOp::Mul
        } else {
            BinOp::Div
        };
        let tmp = parse_factor(&tokens[1..])?;
        let op2 = tmp.0;
        tokens = tmp.1;
        op1 = Ast::BinOp(binop, Box::new(op1), Box::new(op2));
    }

    Ok((op1, tokens))
}

/// Parse numbers, unary minus or groupings of paranthesis.
fn parse_factor(tokens: &[Token]) -> Result<(Ast, &[Token]), ParseError> {
    match tokens.first() {
        Some(t) => match t {
            Token::Num(_) | Token::Minus => {
                let mut sign = 1;
                let mut i = 0;
                while let Some(Token::Minus) = tokens.get(i) {
                    sign *= -1;
                    i += 1;
                }

                match tokens.get(i) {
                    Some(Token::Num(n)) => Ok((Ast::Num(sign * n), &tokens[(i + 1)..])),
                    Some(_) => Err(ParseError::NoNumAfterUnaryMinus),
                    None => Err(ParseError::UnexpectedEOF),
                }
            }

            Token::LParen => {
                let (ast, tokens) = parse_expr(&tokens[1..])?;
                match tokens.first() {
                    Some(Token::RParen) => Ok((ast, &tokens[1..])),
                    Some(_) => Err(ParseError::UnbalancedParanthesis),
                    None => Err(ParseError::UnexpectedEOF),
                }
            }

            _ => Err(ParseError::UnexpectedToken(t.clone())),
        },

        None => Err(ParseError::UnexpectedEOF),
    }
}

/*******************************************************************************
 * Helper functions for walkint the AST. ***************************************
 ******************************************************************************/

/// Traverse the syntax tree [ast] in post-order (recursively).
/// Return a list of the nodes in post-order / reverse polish notation.
#[allow(dead_code)]
fn post_order_traverse(ast: &Ast) -> Vec<AstVal> {
    let mut res = Vec::new();
    post_order_traverse_(ast, &mut res);
    res
}

#[allow(dead_code)]
fn post_order_traverse_(ast: &Ast, acc: &mut Vec<AstVal>) {
    match ast {
        Ast::Num(n) => acc.push(AstVal::Num(*n)),
        Ast::BinOp(s, a, b) => {
            post_order_traverse_(a, acc);
            post_order_traverse_(b, acc);
            acc.push(AstVal::BinOp(s.clone()));
        }
    }
}

/// Traverse the syntax tree [ast] in post-order (iteratively).
/// Return a list of the nodes in post-order / reverse polish notation.
fn post_order_traverse_iterative(ast: &Ast) -> Vec<AstVal> {
    let mut tmp = vec![ast]; //      temporary stack
    let mut res_rev = Vec::new(); // reverse post-order traversal

    while let Some(node) = tmp.pop() {
        match node {
            Ast::Num(n) => res_rev.push(AstVal::Num(*n)),
            Ast::BinOp(s, a, b) => {
                res_rev.push(AstVal::BinOp(s.clone()));

                // push all children
                tmp.push(a);
                tmp.push(b);
            }
        }
    }

    res_rev.into_iter().rev().collect()
}

/*******************************************************************************
 * JITting. ********************************************************************
 ******************************************************************************/

/// Jitted code (native x86_64).
struct Jit {
    code: Vec<u8>,
}

impl fmt::Display for Jit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, b) in self.code.iter().enumerate() {
            if i % 0x10 == 0 && i != 0 {
                writeln!(f)?;
            }
            if i % 0x10 == 0 {
                write!(f, "0x{:010x}:", i)?;
            }
            if i % 0x08 == 0 {
                write!(f, " ")?;
            }
            write!(f, "{:02x} ", b)?;
        }
        Ok(())
    }
}

/// Jit the arithmetic expression given in [ast] down to native x86_64 native
/// machine code. The jit code is implemented as a single function with prologue
/// and epilogue. It uses only the registers rax and rbx (rbx is saved at start
/// of the function and restored at end), and the stack.
fn jit(ast: Ast) -> Jit {
    let mut code: Vec<u8> = Vec::new();

    // function prologue
    code.extend(vec![0x55]); //             push   rbp
    code.extend(vec![0x48, 0x89, 0xe5]); // mov    rbp,rsp
    code.extend(vec![0x53]); //             push   rbx

    jit_(&mut code, ast);

    // function epilogue
    code.extend(vec![0x5b]); // pop    rbx
    code.extend(vec![0x5d]); // pop    rbp
    code.extend(vec![0xc3]); // ret

    // push rax; mov rax, 2; pop rax
    Jit { code }
}

/// Walk the ast and jit each operation.
fn jit_(code: &mut Vec<u8>, ast: Ast) {
    let post_order = post_order_traverse_iterative(&ast);

    for po in post_order.into_iter() {
        match po {
            AstVal::Num(n) => jit_push(code, n),
            AstVal::BinOp(op) => jit_apply_op(code, op),
        }
    }

    // pop final computed value off of the stack
    code.push(0x58); // pop rax
}

/// Push a 64 bit immediate value to the stack (via rax).
fn jit_push(code: &mut Vec<u8>, n: Num) {
    // mov rax, n
    code.extend([0x48, 0xb8]);
    code.extend(n.to_le_bytes());

    // push rax
    code.push(0x50);
}

/// Pop the two top-most items off of the stack, apply the specified binary
/// operator to them, and push the result back to the stack.
fn jit_apply_op(code: &mut Vec<u8>, op: BinOp) {
    code.push(0x5b); // pop rbx
    code.push(0x58); // pop rax
    match op {
        BinOp::Add => code.extend([0x48, 0x01, 0xd8]), //              add rax, rbx
        BinOp::Sub => code.extend([0x48, 0x29, 0xd8]), //              sub rax, rbx
        BinOp::Mul => code.extend([0x48, 0xf7, 0xeb]), //              imul rbx
        BinOp::Div => code.extend([0x48, 0x99, 0x48, 0xf7, 0xfb]), //  cqo (sign extend); idiv rbx
    }
    // result is now always in rax
    code.push(0x50); // push rax
}

// C ffi function definitions needed for the JIT (dynamically marking memory as
// executable).
extern "C" {
    fn mmap(
        addr: *mut ffi::c_void,
        len: usize,
        prot: i32,
        flags: i32,
        fildes: i32,
        off: usize,
    ) -> *mut ffi::c_void;
}

// Memory protetion constants.
const PROT_READ: i32 = 1;
const PROT_WRITE: i32 = 2;
const PROT_EXEC: i32 = 4;

// Mmap flags.
const MAP_ANON: i32 = 0x0020;
const MAP_PRIVATE: i32 = 0x0002;

/// Run the provided jitted code.
fn run_jit(jit: &Jit) -> Num {
    // Allocate rwx memory.
    let mem = unsafe {
        mmap(
            std::ptr::null_mut(),
            jit.code.len(),
            PROT_READ | PROT_WRITE | PROT_EXEC,
            MAP_ANON | MAP_PRIVATE,
            -1,
            0,
        )
    };

    // Copy jitted code into rwx memory.
    unsafe {
        std::ptr::copy_nonoverlapping(jit.code.as_ptr(), mem as *mut u8, jit.code.len());
    }

    // Cast rwx buffer as function pointer and call it.
    let func: extern "C" fn() -> Num = unsafe { std::mem::transmute(mem) };
    func()
}

/*******************************************************************************
 * Interpreting. ***************************************************************
 ******************************************************************************/

/// Evaluate given AST based on reverse-polish notation / post-order traversal
/// stack machine interpreter.
fn eval(ast: &Ast) -> Option<Num> {
    // let post_order = post_order_traverse(ast);
    let post_order = post_order_traverse_iterative(ast);

    let mut stack = Vec::new();
    for po in post_order.into_iter() {
        match po {
            AstVal::Num(n) => stack.push(n),
            AstVal::BinOp(name) => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(match name {
                    BinOp::Add => a.checked_add(b)?,
                    BinOp::Sub => a.checked_sub(b)?,
                    BinOp::Mul => a.checked_mul(b)?,
                    BinOp::Div => a.checked_div(b)?,
                });
            }
        }
    }

    if stack.len() != 1 {
        panic!("Error");
    }

    Some(stack.pop().unwrap())
}

/*******************************************************************************
 * Testing. ********************************************************************
 ******************************************************************************/

#[cfg(test)]
mod tests {
    use super::*;

    use rand::rngs::ThreadRng;
    use rand::Rng;

    /// Create some "random" test cases / arithmetic expressions, evaluate them
    /// both by interpretation and jitting, and make sure that the results agree
    /// with each other.
    #[test]
    fn test_compare_interpreter_jit() {
        let mut rng = rand::thread_rng();

        let ntestcases = 25;
        let mut i = 0;
        while i < ntestcases {
            let expr = get_random_ast(&mut rng, 1);
            eval_and_jit(expr);
            i += 1;
            println!();
        }
    }

    /// Generate a "random" arithmetic expression by choosing between each
    /// possible construction method randomly.
    /// [mindepth] is used to exclude one-level arithmetic expressions which
    /// only consist of a single number and are otherwise relatively frequent.
    fn get_random_ast(rng: &mut ThreadRng, mindepth: usize) -> Ast {
        loop {
            let (ast, depth) = get_random_ast_(rng, 0);
            if depth >= mindepth {
                return ast;
            }
        }
    }

    fn get_random_ast_(rng: &mut ThreadRng, curdepth: usize) -> (Ast, usize) {
        match rng.gen_range(0..2) {
            0 => (Ast::Num(rng.gen_range(-0x1000..0x1000)), curdepth),
            1 => {
                let (a, ad) = get_random_ast_(rng, curdepth + 1);
                let (b, bd) = get_random_ast_(rng, curdepth + 1);
                (
                    Ast::BinOp(
                        match rng.gen_range(0..4) {
                            0 => BinOp::Add,
                            1 => BinOp::Sub,
                            2 => BinOp::Mul,
                            3 => BinOp::Div,
                            _ => panic!("Not reachable"),
                        },
                        Box::new(a),
                        Box::new(b),
                    ),
                    std::cmp::max(ad, bd),
                )
            }
            _ => panic!("Not reachable"),
        }
    }
}

// Debug break: unsafe { std::arch::asm!("int3"); }
