use std::io::{self, Write};

use byteorder::{NativeEndian, WriteBytesExt, ByteOrder};

pub type Category = u8;
const DEFAULT_CATEGORY: Category = 0;

pub trait CharCategory {
    fn char_category(&self, ch: char) -> Category;
    fn invoke(&self, c: Category) -> bool;
    fn group(&self, c: Category) -> bool;
    fn length(&self, c: Category) -> u8;
}

pub struct CharTable {
    pub invokes: Vec<bool>,
    pub groups: Vec<bool>,
    pub lengths: Vec<u8>,
    pub table: [Category; ::std::u16::MAX as usize],
}

impl CharCategory for CharTable {
    fn char_category(&self, ch: char) -> Category {
        let ch = ch as u32;
        if ch < ::std::u16::MAX as u32 {
            self.table[ch as usize]
        } else {
            DEFAULT_CATEGORY
        }
    }

    fn invoke(&self, c: Category) -> bool {
        self.invokes[c as usize]
    }

    fn group(&self, c: Category) -> bool {
        self.groups[c as usize]
    }

    fn length(&self, c: Category) -> u8 {
        self.lengths[c as usize]
    }
}

impl CharTable {
    pub fn encode<W: Write>(&self, mut w: W) -> io::Result<()> {
        let n = self.invokes.len() as u8;
        w.write_u8(n)?;
        for &b in self.invokes.iter() {
            w.write_u8(b as u8)?;
        }
        for &b in self.groups.iter() {
            w.write_u8(b as u8)?;
        }
        for &b in self.lengths.iter() {
            w.write_u8(b)?;
        }
        for &b in self.table.iter() {
            w.write_u8(b)?;
        }
        Ok(())
    }

    pub fn encode_native<W: Write>(&self, w: W) -> io::Result<()> {
        self.encode::<W>(w)
    }
}

pub struct CompiledCharTable<'a> {
    pub n_categories: u8,
    pub invokes: &'a [u8],
    pub groups: &'a [u8],
    pub lengths: &'a [u8],
    pub table: &'a [Category],
}

impl<'a> CharCategory for CompiledCharTable<'a> {
    fn char_category(&self, ch: char) -> Category {
        let ch = ch as u32;
        if ch < ::std::u16::MAX as u32 {
            self.table[ch as usize]
        } else {
            DEFAULT_CATEGORY
        }
    }

    fn invoke(&self, c: Category) -> bool {
        self.invokes[c as usize] != 0
    }

    fn group(&self, c: Category) -> bool {
        self.groups[c as usize] != 0
    }

    fn length(&self, c: Category) -> u8 {
        self.lengths[c as usize]
    }
}

impl<'a> CompiledCharTable<'a> {
    pub unsafe fn decode(bs: &'a [u8]) -> Self {
        let ptr = bs.as_ptr() as *const u8;
        let n = *ptr;
        let ptr = ptr.offset(1);
        let invokes = ::std::slice::from_raw_parts(ptr, n as usize);
        let ptr = ptr.offset(n as isize);
        let groups = ::std::slice::from_raw_parts(ptr, n as usize);
        let ptr = ptr.offset(n as isize);
        let lengths = ::std::slice::from_raw_parts(ptr, n as usize);
        let ptr = ptr.offset(n as isize);
        let table = ::std::slice::from_raw_parts(ptr, ::std::u16::MAX as usize);
        CompiledCharTable {
            n_categories: n,
            invokes: invokes,
            groups: groups,
            lengths: lengths,
            table: table,
        }
    }
}

#[test]
fn test_encode_decode() {
    let mut table = CharTable {
        invokes: vec![true, false, false],
        groups: vec![false, true, false],
        lengths: vec![0, 1, 2],
        table: [DEFAULT_CATEGORY; ::std::u16::MAX as usize],
    };
    table.table['あ' as usize] = 1;
    table.table['a' as usize] = 2;

    let mut buf = Vec::new();
    table.encode(&mut buf).unwrap();

    let compiled = unsafe { CompiledCharTable::decode(&buf) };

    let tests = vec![
        ('0', (true, false, 0)),
        ('あ', (false, true, 1)),
        ('a', (false, false, 2)),
    ];

    for (ch, (i, g, l)) in tests {
        let category = compiled.char_category(ch);
        assert_eq!(category, table.char_category(ch));
        assert_eq!(compiled.invoke(category), i);
        assert_eq!(compiled.group(category), g);
        assert_eq!(compiled.length(category), l);
    }
}
