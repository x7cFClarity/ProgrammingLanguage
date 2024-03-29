use x7cf_cty_pl_lib::parser::lexer::Lexer;

enum Tokens {
    LeftCurlyBrace,
    RightCurlyBrace,
    Abc
}

fn main() {
    let mut lexer = Lexer::new("abc { }");

    let processors = lexer.get_processors();

    processors.push(&|traverser| {
        traverser.read_symbol(10);
        traverser
    });

    lexer.start();
}
