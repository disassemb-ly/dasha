use std::fmt;

use crate::{Addr, DisplayFormat, Format, Size};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Imm {
    U8(u8),
}

impl Imm {
    pub fn size(self) -> Size {
        match self {
            Imm::U8(_) => Size::Byte,
        }
    }
}

impl DisplayFormat for Imm {
    fn fmt(&self, fmt: Format, f: &mut fmt::Formatter) -> fmt::Result {
        match (self, fmt) {
            (Imm::U8(imm), Format::Att) => write!(f, "${:#x}", imm),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Val {
    Addr(Addr),
    Imm(Imm),
}

impl Val {
    pub fn size(self) -> Size {
        match self {
            Val::Addr(addr) => addr.size(),
            Val::Imm(imm) => imm.size(),
        }
    }
}

impl DisplayFormat for Val {
    fn fmt(&self, fmt: Format, f: &mut fmt::Formatter) -> fmt::Result {
        match (self, fmt) {
            (Val::Addr(addr), _) => write!(f, "{}", addr.display(fmt)),
            (Val::Imm(imm), _) => write!(f, "{}", imm.display(fmt)),
        }
    }
}
