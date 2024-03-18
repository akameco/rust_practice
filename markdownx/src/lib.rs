use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Token {
    Heading(u32, String),
    Text(String),
}

pub fn tokenizer(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut buffer = String::new();
    for line in input.lines() {
        let mut chars = line.chars().peekable();
        while let Some(c) = chars.next() {
            match c {
                '#' => {
                    let mut level = 1;
                    while let Some('#') = chars.peek() {
                        chars.next();
                        level += 1;
                    }
                    while let Some(' ') = chars.peek() {
                        chars.next();
                    }

                    tokens.push(Token::Heading(level, chars.collect()));
                    break;
                }

                _ => {
                    buffer.push(c);
                }
            }
        }
        if !buffer.is_empty() {
            tokens.push(Token::Text(buffer.clone()));
            buffer.clear();
        }
    }
    tokens
}

#[derive(Clone, Debug, PartialEq)]
pub enum AstNode {
    Heading(u32, String),
    Text(String),
    Paragraph(Vec<AstNode>),
}

pub fn parse(tokens: &[Token]) -> Vec<AstNode> {
    let mut out = Vec::new();
    for token in tokens {
        match token {
            Token::Heading(level, text) => {
                out.push(AstNode::Heading(level.clone(), text.clone()));
            }
            Token::Text(text) => {
                out.push(AstNode::Text(text.clone()));
            }
        }
    }
    out
}

pub fn generator(ast: &[AstNode]) -> String {
    let mut out = String::new();
    for node in ast {
        match node {
            AstNode::Heading(level, text) => {
                let tag = format!("h{}", level);
                out.push_str(&format!("<{}>{}</{}>", tag, text, tag));
            }
            AstNode::Text(text) => {
                out.push_str(text);
            }
            _ => {
                panic!("#{:?}は定義されていないNodeです", node);
            }
        }
    }
    out
}

#[wasm_bindgen]
pub fn markdownx(input: &str) -> String {
    generator(&parse(&tokenizer(input)))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_tokenizer() {
        let input = "# Title\nHello World\n### SubTitle\nHello World";
        let expect = vec![
            Token::Heading(1, format!("Title")),
            Token::Text(format!("Hello World")),
            Token::Heading(3, format!("SubTitle")),
            Token::Text(format!("Hello World")),
        ];
        assert_eq!(tokenizer(input), expect);
    }

    #[test]
    fn test_parse() {
        let input = "# Title\nHello World\n### SubTitle\nHello World";
        let expect = vec![
            AstNode::Heading(1, format!("Title")),
            AstNode::Text(format!("Hello World")),
            AstNode::Heading(3, format!("SubTitle")),
            AstNode::Text(format!("Hello World")),
        ];
        let input = tokenizer(input);
        assert_eq!(parse(&input), expect);
    }

    #[test]
    fn test_generate_html() {
        let input = "# Title\nHello World\n### SubTitle\nHello World";
        let expect = "<h1>Title</h1>Hello World<h3>SubTitle</h3>Hello World";
        let input = tokenizer(input);
        let input = parse(&input);
        let input = generator(&input);
        assert_eq!(input, expect);
    }
}
