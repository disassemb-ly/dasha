use std::{error, fmt};

use super::{Inst, Op, Reg, Size, Spanning};

// addr mode
enum Mode {
    // 00
    Indirect,
    // 01
    ByteDisp,
    // 10
    LongDisp,
    // 11
    Direct,
}

trait ByteExt {
    fn mode(self) -> Mode;
    fn reg(self, sz: Size) -> Spanning<Reg>;
    fn rm(self, sz: Size) -> Spanning<Reg>;
}

impl<'a> ByteExt for &'a Spanning<u8> {
    fn mode(self) -> Mode {
        match self.0 >> 6 {
            0b00 => Mode::Indirect,
            0b01 => Mode::ByteDisp,
            0b10 => Mode::LongDisp,
            0b11 => Mode::Direct,
            _ => unreachable!(),
        }
    }

    fn reg(self, sz: Size) -> Spanning<Reg> {
        Spanning(
            match (self.0 >> 3 & 0b111, sz) {
                (0b000, Size::Byte) => Reg::Al,
                (0b001, Size::Byte) => Reg::Cl,
                (0b010, Size::Byte) => Reg::Dl,
                (0b011, Size::Byte) => Reg::Bl,
                (0b100, Size::Byte) => Reg::Ah,
                (0b101, Size::Byte) => Reg::Ch,
                (0b110, Size::Byte) => Reg::Dh,
                (0b111, Size::Byte) => Reg::Bh,
                (0b000, Size::Long) => Reg::Eax,
                (0b001, Size::Long) => Reg::Ecx,
                (0b010, Size::Long) => Reg::Edx,
                (0b011, Size::Long) => Reg::Ebx,
                (0b100, Size::Long) => Reg::Esp,
                (0b101, Size::Long) => Reg::Ebp,
                (0b110, Size::Long) => Reg::Esi,
                (0b111, Size::Long) => Reg::Edi,
                _ => unreachable!(),
            },
            self.1,
            self.2,
            Some(0b111000),
        )
    }

    fn rm(self, sz: Size) -> Spanning<Reg> {
        Spanning(
            match (self.0 & 0b111, sz) {
                (0b000, Size::Byte) => Reg::Al,
                (0b001, Size::Byte) => Reg::Cl,
                (0b010, Size::Byte) => Reg::Dl,
                (0b011, Size::Byte) => Reg::Bl,
                (0b100, Size::Byte) => Reg::Ah,
                (0b101, Size::Byte) => Reg::Ch,
                (0b110, Size::Byte) => Reg::Dh,
                (0b111, Size::Byte) => Reg::Bh,
                (0b000, Size::Long) => Reg::Eax,
                (0b001, Size::Long) => Reg::Ecx,
                (0b010, Size::Long) => Reg::Edx,
                (0b011, Size::Long) => Reg::Ebx,
                (0b100, Size::Long) => Reg::Esp,
                (0b101, Size::Long) => Reg::Ebp,
                (0b110, Size::Long) => Reg::Esi,
                (0b111, Size::Long) => Reg::Edi,
                _ => unreachable!(),
            },
            self.1,
            self.2,
            Some(0b111),
        )
    }
}

trait ByteSliceExt<'a> {
    fn mode(self) -> Option<Mode>;
    fn reg(self, sz: Size) -> Result<Spanning<Reg>, Error>;
    fn rm(self, sz: Size) -> Result<Spanning<Op>, Error>;
    fn tail(self) -> Result<&'a [Spanning<u8>], Error>;
}

impl<'a> ByteSliceExt<'a> for &'a [Spanning<u8>] {
    fn mode(self) -> Option<Mode> {
        self.get(0).map(|mrr| match mrr.0 >> 6 {
            0b00 => Mode::Indirect,
            0b01 => Mode::ByteDisp,
            0b10 => Mode::LongDisp,
            0b11 => Mode::Direct,
            _ => unreachable!(),
        })
    }

    fn reg(self, sz: Size) -> Result<Spanning<Reg>, Error> {
        self.get(0).map(|mrr| mrr.reg(sz)).ok_or(Error::ExpectedMrr)
    }

    fn rm(self, sz: Size) -> Result<Spanning<Op>, Error> {
        self.get(0)
            .ok_or(Error::ExpectedMrr)
            .and_then(|mrr| -> Result<_, _> {
                Ok(Spanning(
                    match mrr.mode() {
                        Mode::Indirect => Op::Ind {
                            disp: None,
                            base: Some(mrr.rm(Size::Long)),
                            index: None,
                            scale: None,
                        },
                        Mode::ByteDisp => Op::Ind {
                            disp: Some({
                                let disp = self.get(1).ok_or(Error::ExpectedByteDisp)?.0 as i8;
                                Spanning(disp as _, 0, 0, None)
                            }),
                            base: Some(mrr.rm(Size::Long)),
                            index: None,
                            scale: None,
                        },
                        Mode::LongDisp => Op::Ind {
                            disp: Some({
                                let disp = self
                                    .get(1..5)
                                    .ok_or(Error::ExpectedLongDisp)?
                                    .iter()
                                    .map(|Spanning(byte, _, _, _)| *byte)
                                    .collect::<Vec<_>>();
                                let mut long = [0; 4];
                                long.copy_from_slice(&disp);
                                Spanning(i32::from_le_bytes(long) as _, 0, 0, None)
                            }),
                            base: Some(mrr.rm(Size::Long)),
                            index: None,
                            scale: None,
                        },
                        Mode::Direct => Op::Dir(mrr.rm(sz).0),
                    },
                    mrr.1,
                    mrr.2,
                    Some(0b111),
                ))
            })
    }

    fn tail(self) -> Result<&'a [Spanning<u8>], Error> {
        let mrr = self.get(0).ok_or(Error::ExpectedMrr)?;
        match mrr.mode() {
            Mode::Direct => self.get(1..).ok_or(Error::ExpectedMrr),
            Mode::ByteDisp => self.get(2..).ok_or(Error::ExpectedByteDisp),
            Mode::LongDisp => self.get(5..).ok_or(Error::ExpectedLongDisp),
            Mode::Indirect => self.get(1..).ok_or(Error::ExpectedMrr),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    ExpectedMrr,
    ExpectedByteDisp,
    ExpectedLongDisp,
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::ExpectedMrr => write!(f, "expected mod-reg-r/m byte"),
            Error::ExpectedByteDisp => write!(f, "expected byte disp (1 byte)"),
            Error::ExpectedLongDisp => write!(f, "expected long disp (4 bytes)"),
        }
    }
}

pub fn disasm(mut code: &[Spanning<u8>]) -> Result<Vec<Spanning<Inst>>, Error> {
    let mut insts = vec![];
    while !code.is_empty() {
        let inst;
        (inst, code) = match code {
            [Spanning(0x00, _oss, _osl, _), ref tail @ ..] => (
                Spanning(
                    Inst::AddRegOp(tail.reg(Size::Byte)?, tail.rm(Size::Byte)?),
                    0,
                    0,
                    None,
                ),
                tail.tail()?,
            ),
            _ => unimplemented!("{:?}", code),
        };
        insts.push(inst);
    }
    Ok(insts)
}
