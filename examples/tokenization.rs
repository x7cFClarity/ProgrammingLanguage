use x7cf_cty_pl_lib::parser::lexer::Lexer;

enum Tokens {
    LeftCurlyBrace,
    RightCurlyBrace,
    Abc
}

fn main() {
    let lexer = Lexer::new("abc { }");


}