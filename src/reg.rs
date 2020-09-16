use std::fmt;

use crate::{DisplayFormat, Format};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Reg {
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

impl DisplayFormat for Reg {
    fn fmt(&self, fmt: Format, f: &mut fmt::Formatter) -> fmt::Result {
        match (self, fmt) {
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
