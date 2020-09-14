use crate::{Reg, Scale, Size};

pub trait ByteExt: Into<u8> {
    fn mod_bits(self) -> u8 {
        self.into() >> 6
    }

    fn reg(self, size: Size) -> Reg {
        (self.into() >> 3).rm(size)
    }

    fn rm(self, size: Size) -> Reg {
        match (size, self.into() & 0b111) {
            (Size::Byte, 0b000) => Reg::Al,
            (Size::Byte, 0b001) => Reg::Cl,
            (Size::Byte, 0b010) => Reg::Dl,
            (Size::Byte, 0b011) => Reg::Bl,
            (Size::Byte, 0b100) => Reg::Ah,
            (Size::Byte, 0b101) => Reg::Ch,
            (Size::Byte, 0b110) => Reg::Dh,
            (Size::Byte, 0b111) => Reg::Bh,
            (Size::Long, 0b000) => Reg::Eax,
            (Size::Long, 0b001) => Reg::Ecx,
            (Size::Long, 0b010) => Reg::Edx,
            (Size::Long, 0b011) => Reg::Ebx,
            (Size::Long, 0b100) => Reg::Esp,
            (Size::Long, 0b101) => Reg::Ebp,
            (Size::Long, 0b110) => Reg::Esi,
            (Size::Long, 0b111) => Reg::Edi,
            (_, 0b000..=0b111) => unimplemented!(),
            _ => unreachable!(),
        }
    }

    fn scale(self) -> Scale {
        match self.into() >> 6 {
            0b00 => Scale::One,
            0b01 => Scale::Two,
            0b10 => Scale::Four,
            0b11 => Scale::Eight,
            _ => unreachable!(),
        }
    }

    fn index(self, size: Size) -> Reg {
        self.reg(size)
    }

    fn base(self, size: Size) -> Reg {
        self.rm(size)
    }
}

impl<T> ByteExt for T where T: Into<u8> {}
