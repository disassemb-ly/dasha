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

macro_rules! read_inst {
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
                0x00 => read_inst!(Inst::Add, Size::Byte, Order::Left, &mut i),
                0x01 => read_inst!(Inst::Add, Size::Long, Order::Left, &mut i),
                0x02 => read_inst!(Inst::Add, Size::Byte, Order::Right, &mut i),
                0x03 => read_inst!(Inst::Add, Size::Long, Order::Right, &mut i),
                0x08 => read_inst!(Inst::Or, Size::Byte, Order::Left, &mut i),
                0x09 => read_inst!(Inst::Or, Size::Long, Order::Left, &mut i),
                0x0a => read_inst!(Inst::Or, Size::Byte, Order::Right, &mut i),
                0x0b => read_inst!(Inst::Or, Size::Long, Order::Right, &mut i),
                0x10 => read_inst!(Inst::Adc, Size::Byte, Order::Left, &mut i),
                0x11 => read_inst!(Inst::Adc, Size::Long, Order::Left, &mut i),
                0x12 => read_inst!(Inst::Adc, Size::Byte, Order::Right, &mut i),
                0x13 => read_inst!(Inst::Adc, Size::Long, Order::Right, &mut i),
                0x18 => read_inst!(Inst::Sbb, Size::Byte, Order::Left, &mut i),
                0x19 => read_inst!(Inst::Sbb, Size::Long, Order::Left, &mut i),
                0x1a => read_inst!(Inst::Sbb, Size::Byte, Order::Right, &mut i),
                0x1b => read_inst!(Inst::Sbb, Size::Long, Order::Right, &mut i),
                0x20 => read_inst!(Inst::And, Size::Byte, Order::Left, &mut i),
                0x21 => read_inst!(Inst::And, Size::Long, Order::Left, &mut i),
                0x22 => read_inst!(Inst::And, Size::Byte, Order::Right, &mut i),
                0x23 => read_inst!(Inst::And, Size::Long, Order::Right, &mut i),
                0x28 => read_inst!(Inst::Sub, Size::Byte, Order::Left, &mut i),
                0x29 => read_inst!(Inst::Sub, Size::Long, Order::Left, &mut i),
                0x2a => read_inst!(Inst::Sub, Size::Byte, Order::Right, &mut i),
                0x2b => read_inst!(Inst::Sub, Size::Long, Order::Right, &mut i),
                0x30 => read_inst!(Inst::Xor, Size::Byte, Order::Left, &mut i),
                0x31 => read_inst!(Inst::Xor, Size::Long, Order::Left, &mut i),
                0x32 => read_inst!(Inst::Xor, Size::Byte, Order::Right, &mut i),
                0x33 => read_inst!(Inst::Xor, Size::Long, Order::Right, &mut i),
                0x38 => read_inst!(Inst::Cmp, Size::Byte, Order::Left, &mut i),
                0x39 => read_inst!(Inst::Cmp, Size::Long, Order::Left, &mut i),
                0x3a => read_inst!(Inst::Cmp, Size::Byte, Order::Right, &mut i),
                0x3b => read_inst!(Inst::Cmp, Size::Long, Order::Right, &mut i),
                op => unimplemented!("{:#04x}", op),
            });
        }

        Ok(insts)
    }
}
