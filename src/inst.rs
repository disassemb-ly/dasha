use std::fmt;

use crate::{DisplayFormat, Format, Val};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Inst {
    Push(Val),
    Pop(Val),

    Add(Val, Val),
    Or(Val, Val),
    Adc(Val, Val),
    Sbb(Val, Val),
    And(Val, Val),
    Sub(Val, Val),
    Xor(Val, Val),
    Cmp(Val, Val),
}

impl DisplayFormat for Inst {
    fn fmt(&self, fmt: Format, f: &mut fmt::Formatter) -> fmt::Result {
        macro_rules! write_inst {
            ( $inst:expr, $op:expr ) => {
                write!(
                    f,
                    "{}{} {}",
                    $inst,
                    $op.size().display(fmt),
                    $op.display(fmt)
                )
            };
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
            (Inst::Push(op), Format::Att) => write_inst!("push", op),
            (Inst::Pop(op), Format::Att) => write_inst!("pop", op),
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
