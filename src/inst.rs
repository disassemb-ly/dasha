use std::fmt;

use crate::{Addr, DisplayFormat, Format};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Inst {
    Add(Addr, Addr),
}

impl DisplayFormat for Inst {
    fn fmt(&self, fmt: Format, f: &mut fmt::Formatter) -> fmt::Result {
        match (self, fmt) {
            (Inst::Add(op1, op2), Format::Att) => write!(
                f,
                "add{} {}, {}",
                op1.size().assert_eq(op2.size()).display(fmt),
                op1.display(fmt),
                op2.display(fmt)
            ),
        }
    }
}
