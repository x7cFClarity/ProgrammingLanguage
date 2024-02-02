use std::str::Chars;
use crate::util::stream::ReadStream;

pub struct SymbolReader<'a> {
    symbol_pointer: usize,
    characters_iter: Chars<'a>,
    characters: Vec<char>
}

impl<'a> SymbolReader<'a> {
    pub fn new(characters_iter: Chars) -> Self {
        Self {
            symbol_pointer: 0,
            characters_iter,
            characters: Vec::new()
        }
    }

    pub fn read_symbol(&self, offset: usize) -> Option<char> {
        let pointer = self.symbol_pointer + offset;
        if (self.characters.len() < pointer) {
            Some('a')
        } else {
            None
        }
    }

    pub fn read_next_symbol(capture: bool) {

    }

    pub fn capture_next_symbol() {

    }
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
    source_input: &'a str,
    processors: Vec<&'a dyn Fn(SymbolReader)>
}

impl<'a> Lexer<'a> {
    pub fn new(source_input: &'a str) -> Self {
        Self {
            source_input,
            processors: Vec::new()
        }
    }

    pub fn get_processors(&mut self) -> &mut Vec<&'a dyn Fn(SymbolReader)> {
        &mut self.processors
    }

    pub fn start(&mut self) {
        for func in &self.processors {
            func.call((SymbolReader::new(),));
        }
    }
}

impl<'a, Symbol> ReadStream<Lexeme<'a, Symbol>> for Lexer<'a> {
    fn read_next(&mut self) -> Lexeme<'a, Symbol> {
        todo!()
    }
}
