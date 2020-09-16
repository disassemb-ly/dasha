use std::fmt;

use crate::{DisplayFormat, Format, Reg};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Size {
    Byte,
    Word,
    Long,
}

impl Size {
    pub fn assert_eq(self, other: Self) -> Self {
        assert_eq!(self, other);

        self
    }
}

impl DisplayFormat for Size {
    fn fmt(&self, fmt: Format, f: &mut fmt::Formatter) -> fmt::Result {
        match (self, fmt) {
            (Size::Byte, Format::Att) => write!(f, "b"),
            (Size::Word, Format::Att) => write!(f, "w"),
            (Size::Long, Format::Att) => write!(f, "l"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Scale {
    One,
    Two,
    Four,
    Eight,
}

impl fmt::Display for Scale {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Scale::One => write!(f, "1"),
            Scale::Two => write!(f, "2"),
            Scale::Four => write!(f, "4"),
            Scale::Eight => write!(f, "8"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Offset {
    I8(i8),
    I32(i32),
}

impl fmt::Display for Offset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Offset::I8(offset) if *offset < 0 => write!(f, "-{:#x}", !offset + 1),
            Offset::I8(offset) => write!(f, "{:#x}", offset),
            Offset::I32(offset) if *offset < 0 => write!(f, "-{:#x}", !offset + 1),
            Offset::I32(offset) => write!(f, "{:#x}", offset),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Indirect {
    Base(Size, Reg),
    BaseIndexScale(Size, Reg, Reg, Scale),
    Mem(Size, Offset),
    OffsetBase(Size, Offset, Reg),
    OffsetBaseIndexScale(Size, Offset, Reg, Reg, Scale),
    OffsetIndexScale(Size, Offset, Reg, Scale),
}

impl Indirect {
    pub fn size(&self) -> Size {
        match self {
            Indirect::Base(size, _)
            | Indirect::BaseIndexScale(size, _, _, _)
            | Indirect::Mem(size, _)
            | Indirect::OffsetBase(size, _, _)
            | Indirect::OffsetBaseIndexScale(size, _, _, _, _)
            | Indirect::OffsetIndexScale(size, _, _, _) => *size,
        }
    }
}

impl DisplayFormat for Indirect {
    fn fmt(&self, fmt: Format, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Indirect::Base(_, base) => write!(f, "({})", base.display(fmt)),
            Indirect::BaseIndexScale(_, base, index, scale) => write!(
                f,
                "({}, {}, {})",
                base.display(fmt),
                index.display(fmt),
                scale
            ),
            Indirect::Mem(_, offset) => write!(f, "{}", offset),
            Indirect::OffsetBase(_, offset, base) => write!(f, "{}({})", offset, base.display(fmt)),
            Indirect::OffsetBaseIndexScale(_, offset, base, index, scale) => write!(
                f,
                "{}({}, {}, {})",
                offset,
                base.display(fmt),
                index.display(fmt),
                scale
            ),
            Indirect::OffsetIndexScale(_, offset, index, scale) => {
                write!(f, "{}(, {}, {})", offset, index.display(fmt), scale)
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Addr {
    Direct(Reg),
    Indirect(Indirect),
}

impl Addr {
    pub fn size(&self) -> Size {
        match self {
            Addr::Direct(reg) => reg.size(),
            Addr::Indirect(ind) => ind.size(),
        }
    }
}

impl DisplayFormat for Addr {
    fn fmt(&self, fmt: Format, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Addr::Direct(reg) => write!(f, "{}", reg.display(fmt)),
            Addr::Indirect(ind) => write!(f, "{}", ind.display(fmt)),
        }
    }
}
