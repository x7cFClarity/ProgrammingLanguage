use crate::util::stream::ReadStream;

pub struct Lexer<'a> {
    source_input: &'a str
}

impl<'a> Lexer<'a> {
    pub fn new(source_input: &'a str) -> Self {
        Self {
            source_input
        }
    }
}

impl<'a> ReadStream<()> for Lexer<'a> {
    fn read_next() {

    }
}
