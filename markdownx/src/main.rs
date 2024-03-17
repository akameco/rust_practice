fn main() {
    let input = "# Title\nHello World\n### SubTitle\nHello World";
    println!("{:?}", tokenizer(input));
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
enum Token {
    Heading(u32, String),
    Text(String),
}

fn tokenizer(input: &str) -> Vec<Token> {
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn text_tokenizer() {
        let input = "# Title\nHello World\n### SubTitle\nHello World";
        let expect = vec![
            Token::Heading(1, format!("Title")),
            Token::Text(format!("Hello World")),
            Token::Heading(3, format!("SubTitle")),
            Token::Text(format!("Hello World")),
        ];
        assert_eq!(tokenizer(input), expect);
    }
}
