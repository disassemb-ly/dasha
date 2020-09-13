use crate::Reg;

#[derive(Debug, PartialEq)]
pub enum Addr {
    Direct(Reg),
}
