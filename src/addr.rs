use crate::Reg;

#[derive(Debug, PartialEq)]
pub enum Size {
    Byte,
    Word,
    Long,
}

#[derive(Debug, PartialEq)]
pub enum Indirect {
    Base(Size, Reg),
    Mem(Size, i32),
}

#[derive(Debug, PartialEq)]
pub enum Addr {
    Direct(Reg),
    Indirect(Indirect),
}
