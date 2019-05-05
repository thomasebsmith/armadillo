use std::str::Chars;
use std::iter::Peekable;

fn is_white_space(c: char) -> bool {
    match c {
        '\t' => true,
        '\u{000B}' => true,
        '\u{000C}' => true,
        ' ' => true,
        '\u{00A0}' => true,
        '\u{1680}' => true,
        '\u{2000}' => true,
        '\u{2001}' => true,
        '\u{2002}' => true,
        '\u{2003}' => true,
        '\u{2004}' => true,
        '\u{2005}' => true,
        '\u{2006}' => true,
        '\u{2007}' => true,
        '\u{2008}' => true,
        '\u{2009}' => true,
        '\u{200A}' => true,
        '\u{202F}' => true,
        '\u{205F}' => true,
        '\u{3000}' => true,
        '\u{FEFF}' => true,
        _ => false,
    }
}

fn is_line_terminator(c: char) -> bool {
    match c {
        '\n' => true,
        '\r' => true,
        '\u{2028}' => true,
        '\u{2029}' => true,
        _ => false,
    }
}

pub struct Token {
    pub variant: TokenVariant,
    pub content: String,
}

#[derive(Clone, Copy)]
pub enum TokenVariant {
    LineTerminator,
    WhiteSpace,
}

impl TokenVariant {
    fn take(&self, iter: &mut Peekable<Chars>) -> Option<Token> {
        match *self {
            TokenVariant::LineTerminator => {
                if let Some(&next) = iter.peek() {
                    if is_line_terminator(next) {
                        iter.next();
                        if next == '\r' && iter.next() == Some('\n') {
                            iter.next();
                            Some(Token {
                                variant: *self,
                                content: String::from("\r\n"),
                            })
                        }
                        else {
                            Some(Token {
                                variant: *self,
                                content: next.to_string(),
                            })
                        }
                    }
                    else {
                        None
                    }
                }
                else {
                    None
                }
            },
            TokenVariant::WhiteSpace => {
                if let Some(&next) = iter.peek() {
                    if is_white_space(next) {
                        iter.next();
                        Some(Token {
                            variant: *self,
                            content: next.to_string(),
                        })
                    }
                    else {
                        None
                    }
                }
                else {
                    None
                }
            }
        }
    }
}

pub fn tokenize(code: &str) -> Token {
    Token {
        variant: TokenVariant::WhiteSpace,
        content: String::from(code),
    }
}
