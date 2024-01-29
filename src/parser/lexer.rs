use crate::util::stream::ReadStream;

pub struct Reader {

}

pub struct Lexeme<'a, Symbol> {
    pub symbol: Symbol,
    pub row: usize,
    pub column: usize,
    pub text: &'a str
}

pub enum Error {
    IllegalSymbol
}

pub struct Lexer<'a> {
    source_input: &'a str
}

impl<'a> Lexer<'a> {
    pub fn new(source_input: &'a str) -> Self {
        Self {
            source_input
        }
    }

    pub fn get_processors() {

    }
}

impl<'a, Symbol> ReadStream<Lexeme<'a, Symbol>> for Lexer<'a> {
    fn read_next(&mut self) -> Lexeme<'a, Symbol> {
        todo!()
    }
}
