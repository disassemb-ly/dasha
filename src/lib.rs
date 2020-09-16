mod addr;
mod byte_ext;
mod display;
mod error;
mod inst;
mod iter_ext;
mod reg;

pub use addr::{Addr, Indirect, Offset, Scale, Size};
pub use byte_ext::ByteExt;
pub use display::{DisplayFormat, Format};
pub use error::Error;
pub use inst::Inst;
pub use iter_ext::IterExt;
pub use reg::Reg;

pub struct Dasha;

macro_rules! read_inst {
    ( $inst:path, $iter:expr ) => {
        match $iter.next() {
            Some(modrm) if modrm.mod_bits() == 0b00 && modrm.rm(Size::Long) == Reg::Esp => {
                match $iter.next() {
                    Some(sib)
                        if sib.index(Size::Long) == Reg::Esp
                            && sib.base(Size::Long) == Reg::Esp =>
                    {
                        $inst(
                            Addr::Direct(modrm.reg(Size::Byte)),
                            Addr::Indirect(Indirect::Base(Size::Byte, Reg::Esp)),
                        )
                    }
                    Some(sib)
                        if sib.index(Size::Long) == Reg::Esp
                            && sib.base(Size::Long) == Reg::Ebp =>
                    {
                        $inst(
                            Addr::Direct(modrm.reg(Size::Byte)),
                            Addr::Indirect(Indirect::OffsetIndexScale(
                                Size::Byte,
                                Offset::I32($iter.read_le().ok_or(Error::ExpectedOffsetLong)?),
                                Reg::Eiz,
                                sib.scale(),
                            )),
                        )
                    }
                    Some(sib) if sib.index(Size::Long) == Reg::Esp => $inst(
                        Addr::Direct(modrm.reg(Size::Byte)),
                        Addr::Indirect(Indirect::BaseIndexScale(
                            Size::Byte,
                            sib.base(Size::Long),
                            Reg::Eiz,
                            sib.scale(),
                        )),
                    ),
                    Some(sib) if sib.base(Size::Long) == Reg::Ebp => $inst(
                        Addr::Direct(modrm.reg(Size::Byte)),
                        Addr::Indirect(Indirect::OffsetIndexScale(
                            Size::Byte,
                            Offset::I32($iter.read_le().ok_or(Error::ExpectedOffsetLong)?),
                            sib.index(Size::Long),
                            sib.scale(),
                        )),
                    ),
                    Some(sib) => $inst(
                        Addr::Direct(modrm.reg(Size::Byte)),
                        Addr::Indirect(Indirect::BaseIndexScale(
                            Size::Byte,
                            sib.base(Size::Long),
                            sib.index(Size::Long),
                            sib.scale(),
                        )),
                    ),
                    None => return Err(Error::ExpectedSib),
                }
            }
            Some(modrm) if modrm.mod_bits() == 0b00 && modrm.rm(Size::Long) == Reg::Ebp => $inst(
                Addr::Direct(modrm.reg(Size::Byte)),
                Addr::Indirect(Indirect::Mem(
                    Size::Byte,
                    Offset::I32($iter.read_le().ok_or(Error::ExpectedOffsetLong)?),
                )),
            ),
            Some(modrm) if modrm.mod_bits() == 0b00 => $inst(
                Addr::Direct(modrm.reg(Size::Byte)),
                Addr::Indirect(Indirect::Base(Size::Byte, modrm.rm(Size::Long))),
            ),
            Some(modrm) if modrm.mod_bits() == 0b01 && modrm.rm(Size::Long) == Reg::Esp => {
                match $iter.next() {
                    Some(sib) => $inst(
                        Addr::Direct(modrm.reg(Size::Byte)),
                        Addr::Indirect(Indirect::OffsetBaseIndexScale(
                            Size::Byte,
                            Offset::I8($iter.read_le().ok_or(Error::ExpectedOffsetByte)?),
                            sib.base(Size::Long),
                            sib.index(Size::Long),
                            sib.scale(),
                        )),
                    ),
                    None => return Err(Error::ExpectedSib),
                }
            }
            Some(modrm) if modrm.mod_bits() == 0b01 => $inst(
                Addr::Direct(modrm.reg(Size::Byte)),
                Addr::Indirect(Indirect::OffsetBase(
                    Size::Byte,
                    Offset::I8($iter.read_le().ok_or(Error::ExpectedOffsetByte)?),
                    modrm.rm(Size::Long),
                )),
            ),
            Some(modrm) if modrm.mod_bits() == 0b10 && modrm.rm(Size::Long) == Reg::Esp => {
                match $iter.next() {
                    Some(sib) => $inst(
                        Addr::Direct(modrm.reg(Size::Byte)),
                        Addr::Indirect(Indirect::OffsetBaseIndexScale(
                            Size::Byte,
                            Offset::I32($iter.read_le().ok_or(Error::ExpectedOffsetLong)?),
                            sib.base(Size::Long),
                            sib.index(Size::Long),
                            sib.scale(),
                        )),
                    ),
                    None => return Err(Error::ExpectedSib),
                }
            }
            Some(modrm) if modrm.mod_bits() == 0b10 => $inst(
                Addr::Direct(modrm.reg(Size::Byte)),
                Addr::Indirect(Indirect::OffsetBase(
                    Size::Byte,
                    Offset::I32($iter.read_le().ok_or(Error::ExpectedOffsetLong)?),
                    modrm.rm(Size::Long),
                )),
            ),
            Some(modrm) if modrm.mod_bits() == 0b11 => $inst(
                Addr::Direct(modrm.reg(Size::Byte)),
                Addr::Direct(modrm.rm(Size::Byte)),
            ),
            Some(_) => unimplemented!(), // FIXME(s1g)
            #[allow(unreachable_patterns)]
            Some(_) => unreachable!(),
            None => return Err(Error::ExpectedModRm),
        }
    };
}

impl Dasha {
    pub fn disasm<'a, I: IntoIterator<Item = &'a u8>>(i: I) -> Result<Vec<Inst>, Error> {
        let mut i = i.into_iter();
        let mut insts = vec![];

        while let Some(op) = i.next() {
            insts.push(match op {
                0x00 => read_inst!(Inst::Add, &mut i),
                0x08 => read_inst!(Inst::Or, &mut i),
                0x10 => read_inst!(Inst::Adc, &mut i),
                0x18 => read_inst!(Inst::Sbb, &mut i),
                0x20 => read_inst!(Inst::And, &mut i),
                0x28 => read_inst!(Inst::Sub, &mut i),
                0x30 => read_inst!(Inst::Xor, &mut i),
                0x38 => read_inst!(Inst::Cmp, &mut i),
                _ => unimplemented!(),
            });
        }

        Ok(insts)
    }
}
