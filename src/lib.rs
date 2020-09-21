#![feature(fn_traits)]

mod addr;
mod byte_ext;
mod display;
mod error;
mod inst;
mod iter_ext;
mod reg;
mod val;

pub use addr::{Addr, Indirect, Offset, Scale, Size};
pub use byte_ext::ByteExt;
pub use display::{DisplayFormat, Format};
pub use error::Error;
pub use inst::Inst;
pub use iter_ext::IterExt;
pub use reg::Reg;
pub use val::{Imm, Val};

pub struct Dasha;

macro_rules! read_mrr_inst {
    ( $inst:path, $size:expr, $order:path, $iter:expr ) => {
        match $iter.next() {
            Some(modrm) if modrm.mod_bits() == 0b00 && modrm.rm(Size::Long) == Reg::Esp => {
                match $iter.next() {
                    Some(sib)
                        if sib.index(Size::Long) == Reg::Esp
                            && sib.base(Size::Long) == Reg::Esp =>
                    {
                        $inst.call(
                            $order(
                                Val::Addr(Addr::Direct(modrm.reg($size))),
                                Val::Addr(Addr::Indirect(Indirect::Base($size, Reg::Esp))),
                            )
                            .into(),
                        )
                    }
                    Some(sib)
                        if sib.index(Size::Long) == Reg::Esp
                            && sib.base(Size::Long) == Reg::Ebp =>
                    {
                        $inst.call(
                            $order(
                                Val::Addr(Addr::Direct(modrm.reg($size))),
                                Val::Addr(Addr::Indirect(Indirect::OffsetIndexScale(
                                    $size,
                                    Offset::I32($iter.read_le().ok_or(Error::ExpectedOffsetLong)?),
                                    Reg::Eiz,
                                    sib.scale(),
                                ))),
                            )
                            .into(),
                        )
                    }
                    Some(sib) if sib.index(Size::Long) == Reg::Esp => $inst.call(
                        $order(
                            Val::Addr(Addr::Direct(modrm.reg($size))),
                            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                                $size,
                                sib.base(Size::Long),
                                Reg::Eiz,
                                sib.scale(),
                            ))),
                        )
                        .into(),
                    ),
                    Some(sib) if sib.base(Size::Long) == Reg::Ebp => $inst.call(
                        $order(
                            Val::Addr(Addr::Direct(modrm.reg($size))),
                            Val::Addr(Addr::Indirect(Indirect::OffsetIndexScale(
                                $size,
                                Offset::I32($iter.read_le().ok_or(Error::ExpectedOffsetLong)?),
                                sib.index(Size::Long),
                                sib.scale(),
                            ))),
                        )
                        .into(),
                    ),
                    Some(sib) => $inst.call(
                        $order(
                            Val::Addr(Addr::Direct(modrm.reg($size))),
                            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                                $size,
                                sib.base(Size::Long),
                                sib.index(Size::Long),
                                sib.scale(),
                            ))),
                        )
                        .into(),
                    ),
                    None => return Err(Error::ExpectedSib),
                }
            }
            Some(modrm) if modrm.mod_bits() == 0b00 && modrm.rm(Size::Long) == Reg::Ebp => $inst
                .call(
                    $order(
                        Val::Addr(Addr::Direct(modrm.reg($size))),
                        Val::Addr(Addr::Indirect(Indirect::Mem(
                            $size,
                            Offset::I32($iter.read_le().ok_or(Error::ExpectedOffsetLong)?),
                        ))),
                    )
                    .into(),
                ),
            Some(modrm) if modrm.mod_bits() == 0b00 => $inst.call(
                $order(
                    Val::Addr(Addr::Direct(modrm.reg($size))),
                    Val::Addr(Addr::Indirect(Indirect::Base($size, modrm.rm(Size::Long)))),
                )
                .into(),
            ),
            Some(modrm) if modrm.mod_bits() == 0b01 && modrm.rm(Size::Long) == Reg::Esp => {
                match $iter.next() {
                    Some(sib) => $inst.call(
                        $order(
                            Val::Addr(Addr::Direct(modrm.reg($size))),
                            Val::Addr(Addr::Indirect(Indirect::OffsetBaseIndexScale(
                                $size,
                                Offset::I8($iter.read_le().ok_or(Error::ExpectedOffsetByte)?),
                                sib.base(Size::Long),
                                sib.index(Size::Long),
                                sib.scale(),
                            ))),
                        )
                        .into(),
                    ),
                    None => return Err(Error::ExpectedSib),
                }
            }
            Some(modrm) if modrm.mod_bits() == 0b01 => $inst.call(
                $order(
                    Val::Addr(Addr::Direct(modrm.reg($size))),
                    Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                        $size,
                        Offset::I8($iter.read_le().ok_or(Error::ExpectedOffsetByte)?),
                        modrm.rm(Size::Long),
                    ))),
                )
                .into(),
            ),
            Some(modrm) if modrm.mod_bits() == 0b10 && modrm.rm(Size::Long) == Reg::Esp => {
                match $iter.next() {
                    Some(sib) => $inst.call(
                        $order(
                            Val::Addr(Addr::Direct(modrm.reg($size))),
                            Val::Addr(Addr::Indirect(Indirect::OffsetBaseIndexScale(
                                $size,
                                Offset::I32($iter.read_le().ok_or(Error::ExpectedOffsetLong)?),
                                sib.base(Size::Long),
                                sib.index(Size::Long),
                                sib.scale(),
                            ))),
                        )
                        .into(),
                    ),
                    None => return Err(Error::ExpectedSib),
                }
            }
            Some(modrm) if modrm.mod_bits() == 0b10 => $inst.call(
                $order(
                    Val::Addr(Addr::Direct(modrm.reg($size))),
                    Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                        $size,
                        Offset::I32($iter.read_le().ok_or(Error::ExpectedOffsetLong)?),
                        modrm.rm(Size::Long),
                    ))),
                )
                .into(),
            ),
            Some(modrm) if modrm.mod_bits() == 0b11 => $inst.call(
                $order(
                    Val::Addr(Addr::Direct(modrm.reg($size))),
                    Val::Addr(Addr::Direct(modrm.rm($size))),
                )
                .into(),
            ),
            Some(_) => unreachable!(), // all variations of the mod bits are covered
            None => return Err(Error::ExpectedModRm),
        }
    };
}

macro_rules! read_imm_inst {
    ( $inst:path, $dst:expr, $imm:path, $err:expr, $iter:expr ) => {
        $inst(
            Val::Addr(Addr::Direct($dst)),
            Val::Imm($imm($iter.read_le().ok_or($err)?)),
        )
    };
}

enum Order<T> {
    Left(T, T),
    Right(T, T),
}

impl<T> Into<(T, T)> for Order<T> {
    fn into(self) -> (T, T) {
        match self {
            Order::Left(l, r) => (l, r),
            Order::Right(r, l) => (l, r),
        }
    }
}

impl Dasha {
    pub fn disasm<'a, I: IntoIterator<Item = &'a u8>>(i: I) -> Result<Vec<Inst>, Error> {
        let mut i = i.into_iter();
        let mut insts = vec![];

        while let Some(op) = i.next() {
            insts.push(match op {
                0x00 => read_mrr_inst!(Inst::Add, Size::Byte, Order::Left, &mut i),
                0x01 => read_mrr_inst!(Inst::Add, Size::Long, Order::Left, &mut i),
                0x02 => read_mrr_inst!(Inst::Add, Size::Byte, Order::Right, &mut i),
                0x03 => read_mrr_inst!(Inst::Add, Size::Long, Order::Right, &mut i),
                0x04 => read_imm_inst!(Inst::Add, Reg::Al, Imm::U8, Error::ExpectedImmByte, &mut i),
                0x05 => read_imm_inst!(
                    Inst::Add,
                    Reg::Eax,
                    Imm::U32,
                    Error::ExpectedImmLong,
                    &mut i
                ),
                0x06 => Inst::Push(Val::Addr(Addr::Direct(Reg::Es))),
                0x07 => Inst::Pop(Val::Addr(Addr::Direct(Reg::Es))),
                0x08 => read_mrr_inst!(Inst::Or, Size::Byte, Order::Left, &mut i),
                0x09 => read_mrr_inst!(Inst::Or, Size::Long, Order::Left, &mut i),
                0x0a => read_mrr_inst!(Inst::Or, Size::Byte, Order::Right, &mut i),
                0x0b => read_mrr_inst!(Inst::Or, Size::Long, Order::Right, &mut i),
                0x0c => read_imm_inst!(Inst::Or, Reg::Al, Imm::U8, Error::ExpectedImmByte, &mut i),
                0x0d => {
                    read_imm_inst!(Inst::Or, Reg::Eax, Imm::U32, Error::ExpectedImmLong, &mut i)
                }
                0x0e => Inst::Push(Val::Addr(Addr::Direct(Reg::Cs))),
                0x0f => unimplemented!("two-byte opcode prefix (0x0f)"), // TODO: impl two-byte opcode prefix
                0x10 => read_mrr_inst!(Inst::Adc, Size::Byte, Order::Left, &mut i),
                0x11 => read_mrr_inst!(Inst::Adc, Size::Long, Order::Left, &mut i),
                0x12 => read_mrr_inst!(Inst::Adc, Size::Byte, Order::Right, &mut i),
                0x13 => read_mrr_inst!(Inst::Adc, Size::Long, Order::Right, &mut i),
                0x14 => read_imm_inst!(Inst::Adc, Reg::Al, Imm::U8, Error::ExpectedImmByte, &mut i),
                0x15 => read_imm_inst!(
                    Inst::Adc,
                    Reg::Eax,
                    Imm::U32,
                    Error::ExpectedImmLong,
                    &mut i
                ),
                0x16 => Inst::Push(Val::Addr(Addr::Direct(Reg::Ss))),
                0x17 => Inst::Pop(Val::Addr(Addr::Direct(Reg::Ss))),
                0x18 => read_mrr_inst!(Inst::Sbb, Size::Byte, Order::Left, &mut i),
                0x19 => read_mrr_inst!(Inst::Sbb, Size::Long, Order::Left, &mut i),
                0x1a => read_mrr_inst!(Inst::Sbb, Size::Byte, Order::Right, &mut i),
                0x1b => read_mrr_inst!(Inst::Sbb, Size::Long, Order::Right, &mut i),
                0x1c => read_imm_inst!(Inst::Sbb, Reg::Al, Imm::U8, Error::ExpectedImmByte, &mut i),
                0x1d => read_imm_inst!(
                    Inst::Sbb,
                    Reg::Eax,
                    Imm::U32,
                    Error::ExpectedImmLong,
                    &mut i
                ),
                0x1e => Inst::Push(Val::Addr(Addr::Direct(Reg::Ds))),
                0x1f => Inst::Pop(Val::Addr(Addr::Direct(Reg::Ds))),
                0x20 => read_mrr_inst!(Inst::And, Size::Byte, Order::Left, &mut i),
                0x21 => read_mrr_inst!(Inst::And, Size::Long, Order::Left, &mut i),
                0x22 => read_mrr_inst!(Inst::And, Size::Byte, Order::Right, &mut i),
                0x23 => read_mrr_inst!(Inst::And, Size::Long, Order::Right, &mut i),
                0x24 => read_imm_inst!(Inst::And, Reg::Al, Imm::U8, Error::ExpectedImmByte, &mut i),
                0x25 => read_imm_inst!(
                    Inst::And,
                    Reg::Eax,
                    Imm::U32,
                    Error::ExpectedImmLong,
                    &mut i
                ),
                0x26 => unimplemented!("es segment override prefix (0x26)"),
                0x27 => Inst::Daa,
                0x28 => read_mrr_inst!(Inst::Sub, Size::Byte, Order::Left, &mut i),
                0x29 => read_mrr_inst!(Inst::Sub, Size::Long, Order::Left, &mut i),
                0x2a => read_mrr_inst!(Inst::Sub, Size::Byte, Order::Right, &mut i),
                0x2b => read_mrr_inst!(Inst::Sub, Size::Long, Order::Right, &mut i),
                0x2c => read_imm_inst!(Inst::Sub, Reg::Al, Imm::U8, Error::ExpectedImmByte, &mut i),
                0x2d => read_imm_inst!(
                    Inst::Sub,
                    Reg::Eax,
                    Imm::U32,
                    Error::ExpectedImmLong,
                    &mut i
                ),
                0x2e => unimplemented!("cs segment override prefix (0x2e)"),
                0x2f => Inst::Das,
                0x30 => read_mrr_inst!(Inst::Xor, Size::Byte, Order::Left, &mut i),
                0x31 => read_mrr_inst!(Inst::Xor, Size::Long, Order::Left, &mut i),
                0x32 => read_mrr_inst!(Inst::Xor, Size::Byte, Order::Right, &mut i),
                0x33 => read_mrr_inst!(Inst::Xor, Size::Long, Order::Right, &mut i),
                0x34 => read_imm_inst!(Inst::Xor, Reg::Al, Imm::U8, Error::ExpectedImmByte, &mut i),
                0x35 => read_imm_inst!(
                    Inst::Xor,
                    Reg::Eax,
                    Imm::U32,
                    Error::ExpectedImmLong,
                    &mut i
                ),
                0x36 => unimplemented!("ss segment override prefix (0x36)"),
                0x37 => Inst::Aaa,
                0x38 => read_mrr_inst!(Inst::Cmp, Size::Byte, Order::Left, &mut i),
                0x39 => read_mrr_inst!(Inst::Cmp, Size::Long, Order::Left, &mut i),
                0x3a => read_mrr_inst!(Inst::Cmp, Size::Byte, Order::Right, &mut i),
                0x3b => read_mrr_inst!(Inst::Cmp, Size::Long, Order::Right, &mut i),
                0x3c => read_imm_inst!(Inst::Cmp, Reg::Al, Imm::U8, Error::ExpectedImmByte, &mut i),
                0x3d => read_imm_inst!(
                    Inst::Cmp,
                    Reg::Eax,
                    Imm::U32,
                    Error::ExpectedImmLong,
                    &mut i
                ),
                0x3e => unimplemented!("ds segment override prefix (0x3e)"),
                0x3f => Inst::Aas,
                op @ (0x40..=0x47) => Inst::Inc(Val::Addr(Addr::Direct(op.rm(Size::Long)))),
                op @ (0x48..=0x4f) => Inst::Dec(Val::Addr(Addr::Direct(op.rm(Size::Long)))),
                op @ (0x50..=0x57) => Inst::Push(Val::Addr(Addr::Direct(op.rm(Size::Long)))),
                op @ (0x58..=0x5f) => Inst::Pop(Val::Addr(Addr::Direct(op.rm(Size::Long)))),
                0x60 => Inst::PushA,
                0x61 => Inst::PopA,
                op => unimplemented!("{:#04x}", op),
            });
        }

        Ok(insts)
    }
}
