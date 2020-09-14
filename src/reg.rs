#[derive(Debug, PartialEq)]
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
