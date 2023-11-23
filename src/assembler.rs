use std::fmt::{Debug, Display, Formatter};
use lexpr::{from_str_custom, Number, Value};
use lexpr::cons::ListIter;
use lexpr::parse::{KeywordSyntax, Options};

#[derive(Debug, Clone)]
enum Token {
    Label(Box<str>),
    Symbol(Box<str>),
    Number(u64),
    SymbolOffset(Box<str>, i32),
}

pub fn assemble(text: &str) -> anyhow::Result<()> {
    let sexpression = parse_text(text)?;
    let tokens = parse_program_value_to_tokens(&sexpression)?;

    println!("tokens: {:?}", tokens);

    Ok(())
}

fn parse_text(text: &str) -> lexpr::parse::Result<Value> {
    let options = Options::default()
        .with_keyword_syntax(KeywordSyntax::ColonPostfix);
    from_str_custom(text, options)
}

fn parse_program_value_to_tokens(value: &Value) -> Result<Vec<Vec<Token>>, ParseError> {
    value_to_iter(value)?
        .map(|op_expression| {
            value_to_iter(op_expression)?
                .map(|op_token| parse_token(op_token).ok_or_else(|| ParseError::token(op_token)))
                .collect::<Result<Vec<Token>, ParseError>>()
        }).collect::<Result<Vec<Vec<Token>>, ParseError>>()
}

fn parse_token(value: &Value) -> Option<Token> {
    let result = match value {
        Value::Keyword(s) => Token::Label(s.clone()),
        Value::Symbol(s) => Token::Symbol(s.to_lowercase().into_boxed_str()),
        Value::Number(n) => parse_number_token(n)?,
        Value::Cons(c) => Token::SymbolOffset(c.car().as_symbol()?.to_string().into_boxed_str(), c.cdr().as_i64()? as i32),
        _ => None?
    };

    Some(result)
}

fn value_to_iter(value: &Value) -> Result<ListIter<'_>, ParseError> {
    value.list_iter().ok_or_else(|| ParseError::line(value))
}

fn parse_number_token(n: &Number) -> Option<Token> {
    let u = n.as_u64();
    let s = n.as_i64().map(|x| x as u64);
    u.or(s).map(Token::Number)
}

struct ParseError {
    value: Value,
    object: &'static str,
}

impl ParseError {
    pub fn line(value: &Value) -> ParseError {
        ParseError { value: value.clone(), object: "line" }
    }

    pub fn token(value: &Value) -> ParseError {
        ParseError { value: value.clone(), object: "token" }
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
