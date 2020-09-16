use std::fmt;

use crate::{Addr, DisplayFormat, Format};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Inst {
    Add(Addr, Addr),
    Or(Addr, Addr),
    Adc(Addr, Addr),
    Sbb(Addr, Addr),
    And(Addr, Addr),
    Sub(Addr, Addr),
    Xor(Addr, Addr),
    Cmp(Addr, Addr),
}

impl DisplayFormat for Inst {
    fn fmt(&self, fmt: Format, f: &mut fmt::Formatter) -> fmt::Result {
        macro_rules! write_inst {
            ( $inst:expr, $op1:expr, $op2:expr ) => {
                write!(
                    f,
                    "{}{} {}, {}",
                    $inst,
                    $op1.size().assert_eq($op2.size()).display(fmt),
                    $op1.display(fmt),
                    $op2.display(fmt)
                )
            };
        }

        match (self, fmt) {
            (Inst::Add(op1, op2), Format::Att) => write_inst!("add", op1, op2),
            (Inst::Or(op1, op2), Format::Att) => write_inst!("or", op1, op2),
            (Inst::Adc(op1, op2), Format::Att) => write_inst!("adc", op1, op2),
            (Inst::Sbb(op1, op2), Format::Att) => write_inst!("sbb", op1, op2),
            (Inst::And(op1, op2), Format::Att) => write_inst!("and", op1, op2),
            (Inst::Sub(op1, op2), Format::Att) => write_inst!("sub", op1, op2),
            (Inst::Xor(op1, op2), Format::Att) => write_inst!("xor", op1, op2),
            (Inst::Cmp(op1, op2), Format::Att) => write_inst!("cmp", op1, op2),
        }
    }
}
