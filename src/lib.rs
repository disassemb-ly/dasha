mod addr;
mod byte_ext;
mod error;
mod inst;
mod reg;

pub use addr::Addr;
pub use byte_ext::ByteExt;
pub use error::Error;
pub use inst::Inst;
pub use reg::Reg;

pub struct Dasha;

impl Dasha {
    pub fn disasm<'a, I: IntoIterator<Item = &'a u8>>(i: I) -> Result<Vec<Inst>, Error> {
        let mut i = i.into_iter();
        let mut insts = vec![];

        while let Some(op) = i.next() {
            insts.push(match op {
                0x00 => match i.next() {
                    Some(modrm) if modrm.mod_bits() == 0b11 => {
                        Inst::Add(Addr::Direct(modrm.reg()), Addr::Direct(modrm.rm()))
                    }
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
