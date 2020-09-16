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
    ( $inst:path, $size:expr, $iter:expr ) => {
        match $iter.next() {
            Some(modrm) if modrm.mod_bits() == 0b00 && modrm.rm(Size::Long) == Reg::Esp => {
                match $iter.next() {
                    Some(sib)
                        if sib.index(Size::Long) == Reg::Esp
                            && sib.base(Size::Long) == Reg::Esp =>
                    {
                        $inst(
                            Addr::Direct(modrm.reg($size)),
                            Addr::Indirect(Indirect::Base($size, Reg::Esp)),
                        )
                    }
                    Some(sib)
                        if sib.index(Size::Long) == Reg::Esp
                            && sib.base(Size::Long) == Reg::Ebp =>
                    {
                        $inst(
                            Addr::Direct(modrm.reg($size)),
                            Addr::Indirect(Indirect::OffsetIndexScale(
                                $size,
                                Offset::I32($iter.read_le().ok_or(Error::ExpectedOffsetLong)?),
                                Reg::Eiz,
                                sib.scale(),
                            )),
                        )
                    }
                    Some(sib) if sib.index(Size::Long) == Reg::Esp => $inst(
                        Addr::Direct(modrm.reg($size)),
                        Addr::Indirect(Indirect::BaseIndexScale(
                            $size,
                            sib.base(Size::Long),
                            Reg::Eiz,
                            sib.scale(),
                        )),
                    ),
                    Some(sib) if sib.base(Size::Long) == Reg::Ebp => $inst(
                        Addr::Direct(modrm.reg($size)),
                        Addr::Indirect(Indirect::OffsetIndexScale(
                            $size,
                            Offset::I32($iter.read_le().ok_or(Error::ExpectedOffsetLong)?),
                            sib.index(Size::Long),
                            sib.scale(),
                        )),
                    ),
                    Some(sib) => $inst(
                        Addr::Direct(modrm.reg($size)),
                        Addr::Indirect(Indirect::BaseIndexScale(
                            $size,
                            sib.base(Size::Long),
                            sib.index(Size::Long),
                            sib.scale(),
                        )),
                    ),
                    None => return Err(Error::ExpectedSib),
                }
            }
            Some(modrm) if modrm.mod_bits() == 0b00 && modrm.rm(Size::Long) == Reg::Ebp => $inst(
                Addr::Direct(modrm.reg($size)),
                Addr::Indirect(Indirect::Mem(
                    $size,
                    Offset::I32($iter.read_le().ok_or(Error::ExpectedOffsetLong)?),
                )),
            ),
            Some(modrm) if modrm.mod_bits() == 0b00 => $inst(
                Addr::Direct(modrm.reg($size)),
                Addr::Indirect(Indirect::Base($size, modrm.rm(Size::Long))),
            ),
            Some(modrm) if modrm.mod_bits() == 0b01 && modrm.rm(Size::Long) == Reg::Esp => {
                match $iter.next() {
                    Some(sib) => $inst(
                        Addr::Direct(modrm.reg($size)),
                        Addr::Indirect(Indirect::OffsetBaseIndexScale(
                            $size,
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
                Addr::Direct(modrm.reg($size)),
                Addr::Indirect(Indirect::OffsetBase(
                    $size,
                    Offset::I8($iter.read_le().ok_or(Error::ExpectedOffsetByte)?),
                    modrm.rm(Size::Long),
                )),
            ),
            Some(modrm) if modrm.mod_bits() == 0b10 && modrm.rm(Size::Long) == Reg::Esp => {
                match $iter.next() {
                    Some(sib) => $inst(
                        Addr::Direct(modrm.reg($size)),
                        Addr::Indirect(Indirect::OffsetBaseIndexScale(
                            $size,
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
                Addr::Direct(modrm.reg($size)),
                Addr::Indirect(Indirect::OffsetBase(
                    $size,
                    Offset::I32($iter.read_le().ok_or(Error::ExpectedOffsetLong)?),
                    modrm.rm(Size::Long),
                )),
            ),
            Some(modrm) if modrm.mod_bits() == 0b11 => $inst(
                Addr::Direct(modrm.reg($size)),
                Addr::Direct(modrm.rm($size)),
            ),
            Some(_) => unreachable!(), // all variations of the mod bits are covered
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
                0x00 => read_inst!(Inst::Add, Size::Byte, &mut i),
                0x01 => read_inst!(Inst::Add, Size::Long, &mut i),
                0x08 => read_inst!(Inst::Or, Size::Byte, &mut i),
                0x10 => read_inst!(Inst::Adc, Size::Byte, &mut i),
                0x18 => read_inst!(Inst::Sbb, Size::Byte, &mut i),
                0x20 => read_inst!(Inst::And, Size::Byte, &mut i),
                0x28 => read_inst!(Inst::Sub, Size::Byte, &mut i),
                0x30 => read_inst!(Inst::Xor, Size::Byte, &mut i),
                0x38 => read_inst!(Inst::Cmp, Size::Byte, &mut i),
                _ => unimplemented!(),
            });
        }

        Ok(insts)
    }
}
