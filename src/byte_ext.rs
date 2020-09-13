use crate::Reg;

pub trait ByteExt: Into<u8> {
    fn mod_bits(self) -> u8 {
        self.into() >> 6
    }

    fn reg(self) -> Reg {
        match self.into() >> 3 & 0b111 {
            0b000 => Reg::Al,
            0b001 => Reg::Cl,
            0b010 => Reg::Dl,
            0b011 => Reg::Bl,
            0b100 => Reg::Ah,
            0b101 => Reg::Ch,
            0b110 => Reg::Dh,
            0b111 => Reg::Bh,
            _ => unreachable!(),
        }
    }

    fn rm(self) -> Reg {
        match self.into() & 0b111 {
            0b000 => Reg::Al,
            0b001 => Reg::Cl,
            0b010 => Reg::Dl,
            0b011 => Reg::Bl,
            0b100 => Reg::Ah,
            0b101 => Reg::Ch,
            0b110 => Reg::Dh,
            0b111 => Reg::Bh,
            _ => unreachable!(),
        }
    }
}

impl<T> ByteExt for T where T: Into<u8> {}
