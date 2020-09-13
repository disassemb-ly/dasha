use crate::Reg;

#[derive(Debug, PartialEq)]
pub enum Size {
    Byte,
    Word,
    Long,
}

#[derive(Debug, PartialEq)]
pub enum Indirect {
    Base(Reg),
}

#[derive(Debug, PartialEq)]
pub enum Addr {
    Direct(Reg),
    Indirect(Indirect),
}
