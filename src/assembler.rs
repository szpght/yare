use std::fmt::{Debug, Display, Formatter};
use lexpr::{from_str_custom, Number, Value};
use lexpr::cons::ListIter;
use lexpr::parse::{KeywordSyntax, Options};
use crate::opcodes::mnemonic_to_opcode;

#[derive(Debug, Clone)]
enum Token {
    Label(Box<str>),
    Symbol(Box<str>),
    Number(u64),
    SymbolOffset(Box<str>, i32)
}

#[derive(Debug, Clone)]
enum Operation {
    Opcode(u8),
    Symbol(Box<str>),
}

#[derive(Debug, Clone)]
enum Operand {
    Symbol(Box<str>),
    Register(u8),
    Number(u64),
    RegisterWithOffset(u8, i32),
}

#[derive(Debug, Clone)]
struct Node {
    mnemonic: Operation,
    operands: Vec<Operand>,
}

pub fn assemble(text: &str) -> anyhow::Result<()> {
    let sexpression = parse_text(text)?;
    let operation_expressions = value_to_iter(&sexpression)?;

    println!("operation_expressions: {:?}", operation_expressions);
    
    let mut vectors_of_tokens = Vec::new();
    for op_expression in value_to_iter(&sexpression)? {
        let op_tokens = value_to_iter(op_expression)?
            .map(|op_token| parse_token(op_token).ok_or_else(||ParseError::operand(op_token)))
            .collect::<Result<Vec<Token>, ParseError>>()?;
        vectors_of_tokens.push(op_tokens);
    }
    
    println!("operation_expressions: {:?}", vectors_of_tokens);
    
    
    Ok(())
}

fn parse_text(text: &str) -> lexpr::parse::Result<Value> {
    let options = Options::default()
        .with_keyword_syntax(KeywordSyntax::ColonPostfix);
    from_str_custom(text, options)
}

fn parse_token(value: &Value) -> Option<Token> {
    let result = match value {
        Value::Keyword(s) => Token::Label(s.clone()),
        Value::Symbol(s) => Token::Symbol((s.to_lowercase().into_boxed_str())),
        Value::Number(n) => parse_number_token(n)?,
        Value::Cons(c) => Token::SymbolOffset(c.car().as_symbol()?.to_string().into_boxed_str(), c.cdr().as_i64()? as i32),
        _ => unimplemented!()
    };
    
    Some(result)
}

fn value_to_iter(value: &Value) -> Result<ListIter<'_>, ParseError> {
    value.list_iter().ok_or_else(|| ParseError::line(value))
}

fn parse_sexpression(value: &Value) -> Result<Vec<Node>, ParseError> {
    value
        .list_iter().ok_or_else(|| ParseError::line(value))?
        .map(|instruction_sexpression| parse_line(instruction_sexpression)
            .ok_or_else(|| ParseError::line(instruction_sexpression)))
        .collect::<Result<Vec<_>, _>>()
}

fn replace_virtual_operations(nodes: Vec<Node>) -> Vec<Node> {
    nodes.iter().map(|node| {
        if matches!(node.mnemonic, Operation::Symbol(_)) {
            unimplemented!()
        }
        
        node.clone()
    }).collect()
}

fn parse_line(x: &Value) -> Option<Node> {
    let vector = x.to_ref_vec()?;
    let mnemonic_text = vector.get(0)?.as_symbol()?;
    let mnemonic = match mnemonic_to_opcode(mnemonic_text) {
        Some(opcode) => Operation::Opcode(opcode),
        None => Operation::Symbol(mnemonic_text.to_string().into_boxed_str()),
    };
    let operands = vector.iter().skip(1)
        .map(|operand| parse_operand(operand))
        .collect::<Option<Vec<_>>>()?;

    Some(Node { mnemonic, operands })
}

fn parse_operand(value: &Value) -> Option<Operand> {
    match value {
        Value::Number(n) => parse_number_operand(n),
        Value::Symbol(s) => register_name_to_number(s).map(|reg_num| Operand::Register(reg_num)),// Some(Operand::Symbol(s.clone())),
        Value::Cons(_) => panic!("todo"),
        _ => None
    }
}

fn parse_number_operand(n: &Number) -> Option<Operand> {
    let u = n.as_u64();
    let s = n.as_i64().map(|x| x as u64);
    u.or(s).map(Operand::Number)
}

fn parse_number_token(n: &Number) -> Option<Token> {
    let u = n.as_u64();
    let s = n.as_i64().map(|x| x as u64);
    u.or(s).map(Token::Number)
}

fn register_name_to_number(name: &str) -> Option<u8> {
    let name = name.to_lowercase();
    let name = name.as_str();
    match name {
        "r0" => Some(0),
        "r1" => Some(1),
        "r2" => Some(2),
        "r3" => Some(3),
        "r4" => Some(4),
        "r5" => Some(5),
        "r6" => Some(6),
        "r7" => Some(7),
        "r8" => Some(8),
        "r9" => Some(9),
        "r10" => Some(10),
        "r11" => Some(11),
        "r12" => Some(12),
        "r13" => Some(13),
        "r14" => Some(14),
        "r15" => Some(15),
        _ => None
    }
}

struct ParseError {
    value: Value,
    object: &'static str,
}

impl ParseError {
    pub fn line(value: &Value) -> ParseError {
        ParseError { value: value.clone(), object: "line" }
    }
    
    pub fn operand(value: &Value) -> ParseError {
        ParseError { value: value.clone(), object: "operand" }
    }

    pub fn mnemonic(value: &Value) -> ParseError {
        ParseError { value: value.clone(), object: "mnemonic" }
    }

    fn fmt_impl(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid {}: {}", self.object, self.value)
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt_impl(f)
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt_impl(f)
    }
}

impl std::error::Error for ParseError {}


// fn resolve_registers(nodes: Vec<Node>) -> Vec<Node> {
//     nodes.iter().map(|&node| Node {
//         mnemonic: node.mnemonic,
//         operands: node.operands.iter().map(|op| {
//             match op {
//                 Operand::Register(_) => {}
//                 Operand::Number(_) => {}
//                 Operand::RegisterWithOffset(_, _) => {}
//                 Operand::Symbol(s) => register_name_to_number(s).map(|register|Operand::Register(register)).else
//             }
//         }).collect(),
//     }).collect()
// }

// fn parse_lines(value: &Value) -> Vec<Result<Vec<&Value>, ParseError>> {
//     value_to_iter(value)?
//         .map(|operation_expression| value_to_iter(operation_expression))
//         .collect::<Vec<Result<Vec<&Value>, ParseError>>>()
//     // value_to_iter(value)?
//     //     .map(|line_expression| line_expression.list_iter().ok_or_else(|| ParseError::line(line_expression))?
//     //         .ok_or_else(|| ParseError::line(line_expression))
//     //     )
// }