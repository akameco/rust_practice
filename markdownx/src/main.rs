use markdownx::tokenizer;

fn main() {
    let input = "# Title\nHello World\n### SubTitle\nHello World";
    println!("{:?}", tokenizer(input));
}
