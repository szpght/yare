use std::fmt::{Debug, Display, Formatter};
use lexpr::{from_str_custom, Number, Value};
use lexpr::parse::{KeywordSyntax, Options};
use crate::opcodes::mnemonic_to_opcode;

#[derive(Debug)]
enum Mnemonic {
    Opcode(u8),
    Symbol(Box<str>),
}

#[derive(Debug)]
enum Operand {
    Register(u8),
    Number(u64),
    RegisterWithOffset(u8, i32),
    Symbol(Box<str>),
}

#[derive(Debug)]
struct Node {
    mnemonic: Mnemonic,
    operands: Vec<Operand>,
}

pub fn assemble(text: &str) -> anyhow::Result<()> {
    let sexpression = parse_text(text)?;
    let nodes = parse_sexpression(&sexpression)?;
    let nodes = resolve_registers(nodes);

    Ok(())
}

fn parse_text(text: &str) -> lexpr::parse::Result<Value> {
    let options = Options::default()
        .with_keyword_syntax(KeywordSyntax::ColonPrefix);
    from_str_custom(text, options)
}

fn parse_sexpression(value: &Value) -> Result<Vec<Node>, ParseError> {
    value
        .list_iter().ok_or_else(|| ParseError::line(value))?
        .map(|instruction_sexpression| parse_instruction(instruction_sexpression)
            .ok_or_else(|| ParseError::line(instruction_sexpression)))
        .collect::<Result<Vec<_>, _>>()
}

fn resolve_registers(nodes: Vec<Node>) -> Vec<Node> {
    nodes.iter().map(|&node| Node {
        mnemonic: node.mnemonic,
        operands: node.operands.iter().map(|op| {
            match op {
                Operand::Register(_) => {}
                Operand::Number(_) => {}
                Operand::RegisterWithOffset(_, _) => {}
                Operand::Symbol(s) => register_name_to_number(s).map(|register|Operand::Register(register)).else
            }
        }).collect(),
    }).collect()
}

fn parse_instruction(x: &Value) -> Option<Node> {
    let vector = x.to_ref_vec()?;
    let mnemonic_text = vector.get(0)?.as_symbol()?;
    let mnemonic = match mnemonic_to_opcode(mnemonic_text) {
        Some(opcode) => Mnemonic::Opcode(opcode),
        None => Mnemonic::Symbol(mnemonic_text.to_string().into_boxed_str()),
    };
    let operands = vector.iter().skip(1)
        .map(|operand| parse_operand(operand))
        .collect::<Option<Vec<_>>>()?;

    Some(Node { mnemonic, operands })
}

fn parse_operand(value: &Value) -> Option<Operand> {
    match value {
        Value::Number(n) => parse_number_operand(n),
        Value::Symbol(s) => Some(Operand::Symbol(s.clone())),
        Value::Cons(_) => panic!("todo"),
        _ => None
    }
}

fn parse_number_operand(n: &Number) -> Option<Operand> {
    let u = n.as_u64();
    let s = n.as_i64().map(|x| x as u64);
    u.or(s).map(Operand::Number)
}

fn register_name_to_number(name: &str) -> Option<u8> {
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