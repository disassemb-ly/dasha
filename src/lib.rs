mod addr;
mod byte_ext;
mod error;
mod inst;
mod iter_ext;
mod reg;

pub use addr::{Addr, Indirect, Scale, Size};
pub use byte_ext::ByteExt;
pub use error::Error;
pub use inst::Inst;
pub use iter_ext::IterExt;
pub use reg::Reg;

pub struct Dasha;

impl Dasha {
    pub fn disasm<'a, I: IntoIterator<Item = &'a u8>>(i: I) -> Result<Vec<Inst>, Error> {
        let mut i = i.into_iter();
        let mut insts = vec![];

        while let Some(op) = i.next() {
            insts.push(match op {
                0x00 => match i.next() {
                    Some(modrm) if modrm.mod_bits() == 0b00 && modrm.rm(Size::Long) == Reg::Esp => {
                        match i.next() {
                            Some(sib)
                                if sib.index(Size::Long) == Reg::Esp
                                    && sib.base(Size::Long) == Reg::Esp =>
                            {
                                Inst::Add(
                                    Addr::Direct(modrm.reg(Size::Byte)),
                                    Addr::Indirect(Indirect::Base(Size::Byte, Reg::Esp)),
                                )
                            }
                            Some(sib)
                                if sib.index(Size::Long) == Reg::Esp
                                    && sib.base(Size::Long) == Reg::Ebp =>
                            {
                                Inst::Add(
                                    Addr::Direct(modrm.reg(Size::Byte)),
                                    Addr::Indirect(Indirect::OffsetIndexScale(
                                        Size::Byte,
                                        i.read_le().ok_or(Error::ExpectedOffsetLong)?,
                                        Reg::Eiz,
                                        sib.scale(),
                                    )),
                                )
                            }
                            Some(sib) if sib.index(Size::Long) == Reg::Esp => Inst::Add(
                                Addr::Direct(modrm.reg(Size::Byte)),
                                Addr::Indirect(Indirect::BaseIndexScale(
                                    Size::Byte,
                                    sib.base(Size::Long),
                                    Reg::Eiz,
                                    sib.scale(),
                                )),
                            ),
                            Some(sib) if sib.base(Size::Long) == Reg::Ebp => Inst::Add(
                                Addr::Direct(modrm.reg(Size::Byte)),
                                Addr::Indirect(Indirect::OffsetIndexScale(
                                    Size::Byte,
                                    i.read_le().ok_or(Error::ExpectedOffsetLong)?,
                                    sib.index(Size::Long),
                                    sib.scale(),
                                )),
                            ),
                            Some(sib) => Inst::Add(
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
                    Some(modrm) if modrm.mod_bits() == 0b00 && modrm.rm(Size::Long) == Reg::Ebp => {
                        Inst::Add(
                            Addr::Direct(modrm.reg(Size::Byte)),
                            Addr::Indirect(Indirect::Mem(
                                Size::Byte,
                                i.read_le().ok_or(Error::ExpectedOffsetLong)?,
                            )),
                        )
                    }
                    Some(modrm) if modrm.mod_bits() == 0b00 => Inst::Add(
                        Addr::Direct(modrm.reg(Size::Byte)),
                        Addr::Indirect(Indirect::Base(Size::Byte, modrm.rm(Size::Long))),
                    ),
                    Some(modrm) if modrm.mod_bits() == 0b11 => Inst::Add(
                        Addr::Direct(modrm.reg(Size::Byte)),
                        Addr::Direct(modrm.rm(Size::Byte)),
                    ),
                    Some(_) => unimplemented!(), // FIXME(s1g)
                    #[allow(unreachable_patterns)]
                    Some(_) => unreachable!(),
                    None => return Err(Error::ExpectedModRm),
                },
                _ => unimplemented!(),
            });
        }

        Ok(insts)
    }
}
