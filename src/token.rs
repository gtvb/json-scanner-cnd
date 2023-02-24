#[derive(Debug, Clone)]
pub enum TokenType {
    LBRACE,
    RBRACE,
    LBRACKET,
    RBRACKET,
    COLON,
    COMMA,
    WHITESPACE,
    STRING,
    NUMBER,
    BOOL,
    NULL
}

#[derive(Debug, Clone)]
pub enum TokenValue {
    STRING(String),
    NUMBER(f64),
    BOOL(bool)
}

#[derive(Debug, Clone)]
pub struct Token {
    pub t_type: TokenType,
    pub t_value: Option<TokenValue>,
}

impl Token {
    pub fn new(t_type: TokenType, t_value: Option<TokenValue>) -> Token {
        Token { t_type, t_value }
    }
}
