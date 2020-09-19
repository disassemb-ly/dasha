use std::fmt;

use crate::{DisplayFormat, Format, Size};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Reg {
    // segment registers
    Es,
    Cs,
    Ss,
    Ds,

    // 8-bit registers (byte)
    Al,
    Cl,
    Dl,
    Bl,
    Ah,
    Ch,
    Dh,
    Bh,

    // 32-bit registers (long)
    Eax,
    Ecx,
    Edx,
    Ebx,
    Esp,
    Ebp,
    Esi,
    Edi,

    Eiz,
}

impl Reg {
    pub fn size(&self) -> Size {
        match self {
            Reg::Es | Reg::Cs | Reg::Ss | Reg::Ds => Size::Word,
            Reg::Al | Reg::Cl | Reg::Dl | Reg::Bl | Reg::Ah | Reg::Ch | Reg::Dh | Reg::Bh => {
                Size::Byte
            }
            Reg::Eax
            | Reg::Ecx
            | Reg::Edx
            | Reg::Ebx
            | Reg::Esp
            | Reg::Ebp
            | Reg::Esi
            | Reg::Edi
            | Reg::Eiz => Size::Long, // FIXME(s1g): is %eiz long?
        }
    }
}

impl DisplayFormat for Reg {
    fn fmt(&self, fmt: Format, f: &mut fmt::Formatter) -> fmt::Result {
        match (self, fmt) {
            (Reg::Es, Format::Att) => write!(f, "%es"),
            (Reg::Cs, Format::Att) => write!(f, "%cs"),
            (Reg::Ss, Format::Att) => write!(f, "%ss"),
            (Reg::Ds, Format::Att) => write!(f, "%ds"),
            (Reg::Al, Format::Att) => write!(f, "%al"),
            (Reg::Cl, Format::Att) => write!(f, "%cl"),
            (Reg::Dl, Format::Att) => write!(f, "%dl"),
            (Reg::Bl, Format::Att) => write!(f, "%bl"),
            (Reg::Ah, Format::Att) => write!(f, "%ah"),
            (Reg::Ch, Format::Att) => write!(f, "%ch"),
            (Reg::Dh, Format::Att) => write!(f, "%dh"),
            (Reg::Bh, Format::Att) => write!(f, "%bh"),
            (Reg::Eax, Format::Att) => write!(f, "%eax"),
            (Reg::Ecx, Format::Att) => write!(f, "%ecx"),
            (Reg::Edx, Format::Att) => write!(f, "%edx"),
            (Reg::Ebx, Format::Att) => write!(f, "%ebx"),
            (Reg::Esp, Format::Att) => write!(f, "%esp"),
            (Reg::Ebp, Format::Att) => write!(f, "%ebp"),
            (Reg::Esi, Format::Att) => write!(f, "%esi"),
            (Reg::Edi, Format::Att) => write!(f, "%edi"),
            (Reg::Eiz, Format::Att) => write!(f, "%eiz"),
        }
    }
}
