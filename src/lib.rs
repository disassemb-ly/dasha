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
        let _i = i.into_iter();

        let insts = vec![];

        Ok(insts)
    }
}
