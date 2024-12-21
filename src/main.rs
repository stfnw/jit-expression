use std::ffi;
use std::fmt;

/*******************************************************************************
 * Main. ***********************************************************************
 ******************************************************************************/

fn main() {
    // Read expression from commandline.
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        panic!(
            "Usage: {} 'expression'",
            std::env::current_exe().unwrap().display()
        );
    }
    // e.g. "( ( ( 2 + 3 ) * 4 ) + ( 10 / 5 ) )"

    let ast = parse(&args[1]);
    eval_and_jit(ast);
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
        println!("JIT CODE: {}", jitcode);
        let res_jit = run_jit(&jitcode);
        println!("res_jit:         {}", res_jit);

        assert_eq!(res_interpreter, res_jit);
    } else {
        println!("Arithmetic error occurred (bounds over/underflow, divide by zero, etc.)!");
    }
}

/*******************************************************************************
 * Lexing. *********************************************************************
 ******************************************************************************/

/// Tokenize an expression string.
fn lex(expr: &str) -> Vec<&str> {
    expr.split_whitespace().collect()
}

/*******************************************************************************
 * Parsing. ********************************************************************
 ******************************************************************************/

/// (Abstract) syntax tree of arithmetic expressions.
enum Ast {
    Num(i64),
    BinOp(String, Box<Ast>, Box<Ast>),
}

impl fmt::Display for Ast {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Ast::Num(n) => write!(f, "{}", n),
            Ast::BinOp(op, a, b) => write!(f, "({} {} {})", a, op, b),
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
    Num(i64),
    Op(String),
}

/// Compare the current token against an expected one; if it matches, consume
/// the current token; if not, panic.
fn expect(tokens: &Vec<&str>, i: &mut usize, t: &str) {
    if tokens[*i] == t {
        *i += 1;
    } else {
        panic!("Unexpected token {} != {}", tokens[*i], t);
    }
}

/// Parse an arithmetic expression into its (abstract) syntax tree.
/// AST := NUM | (AST + AST) | (AST - AST) | (AST * AST) | (AST / AST)
/// Note: paranthesis are *not* optional. This lexer/parser is very dumb!
/// TODO make lexer and parser less dumb
fn parse(expr: &str) -> Ast {
    let expr = expr.replace("(", " ( ").replace(")", " ) ");
    let tokens = lex(&expr);
    let mut pos = 0;
    parse_(&tokens, &mut pos)
}

/// Actual recursive descent parser.
fn parse_(tokens: &Vec<&str>, i: &mut usize) -> Ast {
    match tokens[*i] {
        "(" => {
            expect(tokens, i, "(");
            let a = parse_(tokens, i);
            let op = tokens[*i];
            *i += 1;
            let b = parse_(tokens, i);
            expect(tokens, i, ")");
            match op {
                "+" | "-" | "*" | "/" => Ast::BinOp(op.to_string(), Box::new(a), Box::new(b)),
                _ => panic!("Unexpected operator: {}", op),
            }
        }
        n => {
            *i += 1;
            let num = n.parse::<i64>().unwrap();
            Ast::Num(num)
        }
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
            acc.push(AstVal::Op(s.clone()));
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
                res_rev.push(AstVal::Op(s.clone()));

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

// TODO implement better hexdump; add linebreaks
impl fmt::Display for Jit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for b in &self.code {
            write!(f, "\\x{:02x}", b)?;
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
            AstVal::Op(name) => jit_apply_op(code, name.as_str()),
        }
    }

    // pop final computed value off of the stack
    code.push(0x58); // pop rax
}

/// Push a 64 bit immediate value to the stack (via rax).
fn jit_push(code: &mut Vec<u8>, n: i64) {
    // mov rax, n
    code.extend([0x48, 0xb8]);
    code.extend(n.to_le_bytes());

    // push rax
    code.push(0x50);
}

/// Pop the two top-most items off of the stack, apply the specified binary
/// operator to them, and push the result back to the stack.
fn jit_apply_op(code: &mut Vec<u8>, op: &str) {
    code.push(0x5b); // pop rbx
    code.push(0x58); // pop rax
    match op {
        "+" => code.extend([0x48, 0x01, 0xd8]), //              add rax, rbx
        "-" => code.extend([0x48, 0x29, 0xd8]), //              sub rax, rbx
        "*" => code.extend([0x48, 0xf7, 0xeb]), //              imul rbx
        "/" => code.extend([0x48, 0x99, 0x48, 0xf7, 0xfb]), //  cqo (sign extend); idiv rbx
        _ => panic!("Unknown operator {}", op),
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
fn run_jit(jit: &Jit) -> i64 {
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
    let func: extern "C" fn() -> i64 = unsafe { std::mem::transmute(mem) };
    func()
}

/*******************************************************************************
 * Interpreting. ***************************************************************
 ******************************************************************************/

/// Evaluate given AST based on reverse-polish notation / post-order traversal
/// stack machine interpreter.
fn eval(ast: &Ast) -> Option<i64> {
    // let post_order = post_order_traverse(ast);
    let post_order = post_order_traverse_iterative(ast);

    let mut stack = Vec::new();
    for po in post_order.into_iter() {
        match po {
            AstVal::Num(n) => stack.push(n),
            AstVal::Op(name) => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(match name.as_str() {
                    "+" => a.checked_add(b)?,
                    "-" => a.checked_sub(b)?,
                    "*" => a.checked_mul(b)?,
                    "/" => a.checked_div(b)?,
                    _ => panic!("Unknown operator {}", name),
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
                            0 => "+",
                            1 => "-",
                            2 => "*",
                            3 => "/",
                            _ => panic!("Not reachable"),
                        }
                        .to_string(),
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
