use std::fmt;

use crate::{Addr, DisplayFormat, Format};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Inst {
    Add(Addr, Addr),
}

impl DisplayFormat for Inst {
    fn fmt(&self, fmt: Format, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Inst::Add(op1, op2) => write!(f, "add {}, {}", op1.display(fmt), op2.display(fmt)), // FIXME(s1g): at&t suffixes insts with size
        }
    }
}
