use std::{fmt::Display, iter::Peekable, str::Chars};

// 自定义 Result 类型
type Result<T> = std::result::Result<T, ExprError>;

// 自定义错误类型
#[derive(Debug, PartialEq)]
pub enum ExprError {
    Parse(String),
}

impl std::error::Error for ExprError {}

impl Display for ExprError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Parse(s) => write!(f, "{}", s),
        }
    }
}

// Token 表示，数字、运算符号、括号
#[derive(Debug, Clone, Copy)]
enum Token {
    Number(i32),
    Plus,           // 加
    Minus,          // 减
    Multiply,       // 乘
    Divide,         // 除
    Power,          // 幂
    LeftParen,      // 左括号
    RightParen,     // 右括号
}

// 左结合
const ASSOC_LEFT: i32 = 0;
// 右结合
const ASSOC_RIGHT: i32 = 1;

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Token::Number(n) => n.to_string(),
                Token::Plus => "+".to_string(),
                Token::Minus => "-".to_string(),
                Token::Multiply => "*".to_string(),
                Token::Divide => "/".to_string(),
                Token::Power => "^".to_string(),
                Token::LeftParen => "(".to_string(),
                Token::RightParen => ")".to_string(),
            }
        )
    }
}

impl Token {
    // 判断是不是运算符号
    fn is_operator(&self) -> bool {
        matches!(self, Token::Plus | Token::Minus | Token::Multiply | Token::Divide | Token::Power)
    }

    // 获取运算符的优先级
    fn precedence(&self) -> i32 {
        match self {
            Token::Plus | Token::Minus => 1,
            Token::Multiply | Token::Divide => 2,
            Token::Power => 3,
            _ => 0,
        }
    }

    // 获取运算符的结合性
    fn assoc(&self) -> i32 {
        match self {
            Token::Power => ASSOC_RIGHT,
            _ => ASSOC_LEFT,
        }
    }

    // 根据当前运算符进行计算
    fn computer(&self, l: i32, r: i32) -> Option<i32> {
        match self {
            Token::Plus => Some(l + r),
            Token::Minus => Some(l - r),
            Token::Multiply => Some(l * r),
            Token::Divide => Some(l / r),
            Token::Power => Some(l.pow(r as u32)),
            _ => None,
        }
    }
}

struct Tokenizer<'a> {
    tokens: Peekable<Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            tokens: input.chars().peekable(),
        }
    }

    // 消除空白字符
    fn consume_whitespace(&mut self) {
        while let Some(&c) = self.tokens.peek() {
            if c.is_whitespace() {
                self.tokens.next();
            } else {
                break;
            }
        }
    }

    // 扫描数字
    fn scan_number(&mut self) -> Option<Token> {
        let mut num = String::new();
        while let Some(&c) = self.tokens.peek() {
            if c.is_numeric() {
                num.push(c);
                self.tokens.next();
            } else {
                break;
            }
        }

        match num.parse() {
            Ok(n) => Some(Token::Number(n)),
            Err(_) => None,
        }
    }

    // 扫描运算符号
    fn scan_operator(&mut self) -> Option<Token> {
        match self.tokens.next() {
            Some('+') => Some(Token::Plus),
            Some('-') => Some(Token::Minus),
            Some('*') => Some(Token::Multiply),
            Some('/') => Some(Token::Divide),
            Some('^') => Some(Token::Power),
            Some('(') => Some(Token::LeftParen),
            Some(')') => Some(Token::RightParen),
            _ => None,
        }
    }
}


// 实现 Iterator 接口，使 Tokenizer 可以通过 for 循环遍历
impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        // 消除前面的空格
        self.consume_whitespace();
        // 解析当前位置的 Token 类型
        match self.tokens.peek() {
            Some(&c) if c.is_numeric() => self.scan_number(),
            Some(_) => self.scan_operator(),
            None => None,
        }
    }
}

pub struct Expr<'a> {
    iter: Peekable<Tokenizer<'a>>,
}

impl<'a> Expr<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            iter: Tokenizer::new(input).peekable(),
        }
    }

    // 计算表达式，获取结果
    pub fn eval(&mut self) -> Result<i32> {
        let result = self.compute_expr(1)?;

        if self.iter.peek().is_some() {
            return Err(ExprError::Parse("Unexpect end of expr".into()));
        }
        Ok(result)
    }

    fn compute_atom(&mut self) -> Result<i32> {
        match self.iter.peek() {
            // 如果是数字的话，直接返回
            Some(Token::Number(n)) => {
                let val = *n;
                self.iter.next();
                Ok(val)
            }

            // 如果是左括号的话，递归计算括号内的值
            Some(Token::LeftParen) => {
                self.iter.next();
                let result = self.compute_expr(1)?;
                if let Some(Token::RightParen) = self.iter.next() {
                    Ok(result)
                } else {
                    Err(ExprError::Parse("Unexpected character".into()))
                }
            }

            _ => Err(ExprError::Parse("Expecting a number or left parenthesis".into()))
        }
    }

    fn compute_expr(&mut self, min_precedence: i32) -> Result<i32> {
        let mut atom_lhs = self.compute_atom()?;

        loop {
            if let Some(&token) = self.iter.peek() {

                // 1. Token 一定是运算符
                // 2. Token 的优先级必须大于等于 min_precedence
                if !token.is_operator() || token.precedence() < min_precedence {
                    break;
                }

                let mut next_precedence = token.precedence();
                if token.assoc() == ASSOC_LEFT {
                    next_precedence += 1;
                }

                self.iter.next();

                // 递归计算右边的表达式
                let atom_rhs = self.compute_expr(next_precedence)?;

                // 得到了两边的值，进行计算
                match token.computer(atom_lhs, atom_rhs) {
                    Some(val) => atom_lhs = val,
                    None => return Err(ExprError::Parse("Unexpected expr".into())),
                }
            } else {
                break;
            }
        }

        Ok(atom_lhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval() {
        let input = "92 + 5 + 5 * 27 - (92 - 12) / 4 + 26";
        let mut expr = Expr::new(input);
        assert_eq!(expr.eval(), Ok(238));
    }
}
