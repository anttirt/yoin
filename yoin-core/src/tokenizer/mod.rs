use std::iter::Iterator;
use std::str::Split;
use std::fmt;
use std::borrow::Borrow;

mod lattice;
use self::lattice::{Lattice, Node, NodeKind};
use sysdic::SysDic;
use dic::FstDic;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token<'a> {
    start: usize,
    surface: &'a str,
    contents: &'a str,
}

impl<'a> Token<'a> {
    fn new(node: Node<'a>) -> Self {
        let Node { start, kind } = node;
        let (surface, contents) = match kind {
            NodeKind::BOS | NodeKind::EOS => unreachable!(),
            NodeKind::Known(morph) => (morph.surface, morph.contents),
            NodeKind::Unknown(surface, entry) => (surface, entry.contents),
        };

        Token {
            start: start,
            surface: surface,
            contents: contents,
        }
    }

    pub fn surface(&self) -> &'a str {
        self.surface
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.start + self.surface().len()
    }

    pub fn features(&self) -> FeatureIter<'a> {
        FeatureIter(self.contents.split(','))
    }
}

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\t{}", self.surface, self.contents)
    }
}

pub struct FeatureIter<'a>(Split<'a, char>);

impl<'a> Iterator for FeatureIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

pub struct Tokenizer<T: Borrow<[u8]>> {
    sysdic: SysDic,
    udic: Option<FstDic<T>>,
}

impl<T: Borrow<[u8]>> Tokenizer<T> {
    pub fn new(sysdic: SysDic) -> Self {
        Tokenizer { sysdic: sysdic, udic: None }
    }

    pub fn with_udic<U: Borrow<[u8]>>(self, udic: FstDic<U>) -> Tokenizer<U> {
        Tokenizer { sysdic: self.sysdic, udic: Some(udic) }
    }

    pub fn tokenize<'a>(&'a self, input: &'a str) -> Vec<Token<'a>> {
        let la = Lattice::build(input, &self.sysdic, self.udic.as_ref());
        la.into_output().into_iter().map(|node| Token::new(node)).collect()
    }
}
