use crate::Addr;

#[derive(Debug, PartialEq)]
pub enum Inst {
    Add(Addr, Addr),
}
