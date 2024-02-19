use lexer::grammar::ScriptParser;
use lexer::token::Token;
use logos::Logos;

fn main() {
    let source_code = "var a = 42;
var b = 23;

# a comment
print (a - b);";

    let lexer = Token::lexer(&source_code[..]);
    let parser = ScriptParser::new();
    let ast = parser.parse(lexer).unwrap();

    println!("{:?}", ast);
}
