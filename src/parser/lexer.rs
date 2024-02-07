use std::cell::RefCell;
use std::ops::DerefMut;
use std::rc::Rc;
use std::str::Chars;
use crate::util::stream::ReadStream;

pub struct SymbolReader<'a> {
    symbol_pointer: usize,
    characters_iter: RefCell<Chars<'a>>
}

impl<'a> SymbolReader<'a> {
    pub fn new(start_index: usize, characters_iter: RefCell<Chars<'a>>) -> Self {
        Self {
            symbol_pointer: start_index,
            characters_iter
        }
    }

    pub fn read_symbol(&self, offset: usize) -> Option<char> {
        println!("Reading symbol {}", offset);
        None
    }

    pub fn capture_next_symbol(&mut self) -> Option<char> {
        return self.characters_iter.borrow_mut().next();
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
    source_characters: RefCell<Chars<'a>>,
    processors: Vec<&'a dyn Fn(SymbolReader) -> SymbolReader>,
    current_index: usize
}

impl<'a> Lexer<'a> {
    pub fn new(source_input: &'a str) -> Self {
        Self {
            source_characters: RefCell::new(source_input.chars()),
            processors: Vec::new(),
            current_index: 0
        }
    }

    pub fn get_processors(&mut self) -> &mut Vec<&'a dyn Fn(SymbolReader) -> SymbolReader> {
        &mut self.processors
    }

    pub fn start(&mut self) {
        for func in &self.processors {
            func.call((SymbolReader::new(self.current_index, self.source_characters.clone()),));
        }
    }
}

impl<'a, Symbol> ReadStream<Lexeme<'a, Symbol>> for Lexer<'a> {
    fn read_next(&mut self) -> Lexeme<'a, Symbol> {
        todo!()
    }
}
