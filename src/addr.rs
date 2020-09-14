use crate::Reg;

#[derive(Debug, PartialEq)]
pub enum Size {
    Byte,
    Word,
    Long,
}

#[derive(Debug, PartialEq)]
pub enum Scale {
    One,
    Two,
    Four,
    Eight,
}

#[derive(Debug, PartialEq)]
pub enum Indirect {
    Base(Size, Reg),
    BaseIndexScale(Size, Reg, Reg, Scale),
    Mem(Size, i32),
    OffsetIndexScale(Size, i128, Reg, Scale),
}

#[derive(Debug, PartialEq)]
pub enum Addr {
    Direct(Reg),
    Indirect(Indirect),
}
