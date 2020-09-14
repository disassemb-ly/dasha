use dasha::{Addr, Dasha, Error, Indirect, Inst, Reg, Size};

#[test]
fn test_direct_addressing() {
    assert_eq!(
        Dasha::disasm(&[0x00, 0xc0]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Al),
            Addr::Direct(Reg::Al),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0xc1]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Al),
            Addr::Direct(Reg::Cl),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0xc2]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Al),
            Addr::Direct(Reg::Dl),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0xc3]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Al),
            Addr::Direct(Reg::Bl),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0xc4]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Al),
            Addr::Direct(Reg::Ah),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0xc5]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Al),
            Addr::Direct(Reg::Ch),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0xc6]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Al),
            Addr::Direct(Reg::Dh),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0xc7]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Al),
            Addr::Direct(Reg::Bh),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0xc8]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Cl),
            Addr::Direct(Reg::Al),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0xc9]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Cl),
            Addr::Direct(Reg::Cl),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0xca]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Cl),
            Addr::Direct(Reg::Dl),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0xcb]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Cl),
            Addr::Direct(Reg::Bl),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0xcc]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Cl),
            Addr::Direct(Reg::Ah),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0xcd]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Cl),
            Addr::Direct(Reg::Ch),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0xce]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Cl),
            Addr::Direct(Reg::Dh),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0xcf]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Cl),
            Addr::Direct(Reg::Bh),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0xd0]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Dl),
            Addr::Direct(Reg::Al),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0xd1]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Dl),
            Addr::Direct(Reg::Cl),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0xd2]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Dl),
            Addr::Direct(Reg::Dl),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0xd3]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Dl),
            Addr::Direct(Reg::Bl),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0xd4]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Dl),
            Addr::Direct(Reg::Ah),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0xd5]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Dl),
            Addr::Direct(Reg::Ch),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0xd6]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Dl),
            Addr::Direct(Reg::Dh),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0xd7]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Dl),
            Addr::Direct(Reg::Bh),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0xd8]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Bl),
            Addr::Direct(Reg::Al),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0xd9]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Bl),
            Addr::Direct(Reg::Cl),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0xda]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Bl),
            Addr::Direct(Reg::Dl),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0xdb]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Bl),
            Addr::Direct(Reg::Bl),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0xdc]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Bl),
            Addr::Direct(Reg::Ah),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0xdd]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Bl),
            Addr::Direct(Reg::Ch),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0xde]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Bl),
            Addr::Direct(Reg::Dh),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0xdf]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Bl),
            Addr::Direct(Reg::Bh),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0xe0]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Ah),
            Addr::Direct(Reg::Al),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0xe1]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Ah),
            Addr::Direct(Reg::Cl),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0xe2]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Ah),
            Addr::Direct(Reg::Dl),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0xe3]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Ah),
            Addr::Direct(Reg::Bl),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0xe4]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Ah),
            Addr::Direct(Reg::Ah),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0xe5]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Ah),
            Addr::Direct(Reg::Ch),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0xe6]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Ah),
            Addr::Direct(Reg::Dh),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0xe7]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Ah),
            Addr::Direct(Reg::Bh),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0xe8]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Ch),
            Addr::Direct(Reg::Al),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0xe9]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Ch),
            Addr::Direct(Reg::Cl),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0xea]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Ch),
            Addr::Direct(Reg::Dl),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0xeb]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Ch),
            Addr::Direct(Reg::Bl),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0xec]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Ch),
            Addr::Direct(Reg::Ah),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0xed]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Ch),
            Addr::Direct(Reg::Ch),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0xee]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Ch),
            Addr::Direct(Reg::Dh),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0xef]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Ch),
            Addr::Direct(Reg::Bh),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0xf0]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Dh),
            Addr::Direct(Reg::Al),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0xf1]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Dh),
            Addr::Direct(Reg::Cl),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0xf2]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Dh),
            Addr::Direct(Reg::Dl),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0xf3]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Dh),
            Addr::Direct(Reg::Bl),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0xf4]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Dh),
            Addr::Direct(Reg::Ah),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0xf5]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Dh),
            Addr::Direct(Reg::Ch),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0xf6]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Dh),
            Addr::Direct(Reg::Dh),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0xf7]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Dh),
            Addr::Direct(Reg::Bh),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0xf8]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Bh),
            Addr::Direct(Reg::Al),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0xf9]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Bh),
            Addr::Direct(Reg::Cl),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0xfa]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Bh),
            Addr::Direct(Reg::Dl),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0xfb]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Bh),
            Addr::Direct(Reg::Bl),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0xfc]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Bh),
            Addr::Direct(Reg::Ah),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0xfd]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Bh),
            Addr::Direct(Reg::Ch),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0xfe]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Bh),
            Addr::Direct(Reg::Dh),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0xff]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Bh),
            Addr::Direct(Reg::Bh),
        )])
    );
}

#[test]
fn test_indirect_base_addressing() {
    assert_eq!(
        Dasha::disasm(&[0x00, 0x00]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Al),
            Addr::Indirect(Indirect::Base(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x01]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Al),
            Addr::Indirect(Indirect::Base(Reg::Ecx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x02]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Al),
            Addr::Indirect(Indirect::Base(Reg::Edx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x03]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Al),
            Addr::Indirect(Indirect::Base(Reg::Ebx)),
        )])
    );
    assert_eq!(Dasha::disasm(&[0x00, 0x04]), Err(Error::ExpectedSib));
    assert_eq!(Dasha::disasm(&[0x00, 0x05]), Err(Error::ExpectedOffsetLong));
    assert_eq!(
        Dasha::disasm(&[0x00, 0x06]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Al),
            Addr::Indirect(Indirect::Base(Reg::Esi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x07]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Al),
            Addr::Indirect(Indirect::Base(Reg::Edi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x08]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Cl),
            Addr::Indirect(Indirect::Base(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x09]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Cl),
            Addr::Indirect(Indirect::Base(Reg::Ecx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x0a]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Cl),
            Addr::Indirect(Indirect::Base(Reg::Edx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x0b]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Cl),
            Addr::Indirect(Indirect::Base(Reg::Ebx)),
        )])
    );
    assert_eq!(Dasha::disasm(&[0x00, 0x0c]), Err(Error::ExpectedSib));
    assert_eq!(Dasha::disasm(&[0x00, 0x0d]), Err(Error::ExpectedOffsetLong));
    assert_eq!(
        Dasha::disasm(&[0x00, 0x0e]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Cl),
            Addr::Indirect(Indirect::Base(Reg::Esi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x0f]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Cl),
            Addr::Indirect(Indirect::Base(Reg::Edi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x10]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Dl),
            Addr::Indirect(Indirect::Base(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x11]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Dl),
            Addr::Indirect(Indirect::Base(Reg::Ecx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x12]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Dl),
            Addr::Indirect(Indirect::Base(Reg::Edx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x13]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Dl),
            Addr::Indirect(Indirect::Base(Reg::Ebx)),
        )])
    );
    assert_eq!(Dasha::disasm(&[0x00, 0x14]), Err(Error::ExpectedSib));
    assert_eq!(Dasha::disasm(&[0x00, 0x15]), Err(Error::ExpectedOffsetLong));
    assert_eq!(
        Dasha::disasm(&[0x00, 0x16]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Dl),
            Addr::Indirect(Indirect::Base(Reg::Esi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x17]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Dl),
            Addr::Indirect(Indirect::Base(Reg::Edi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x18]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Bl),
            Addr::Indirect(Indirect::Base(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x19]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Bl),
            Addr::Indirect(Indirect::Base(Reg::Ecx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x1a]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Bl),
            Addr::Indirect(Indirect::Base(Reg::Edx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x1b]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Bl),
            Addr::Indirect(Indirect::Base(Reg::Ebx)),
        )])
    );
    assert_eq!(Dasha::disasm(&[0x00, 0x1c]), Err(Error::ExpectedSib));
    assert_eq!(Dasha::disasm(&[0x00, 0x1d]), Err(Error::ExpectedOffsetLong));
    assert_eq!(
        Dasha::disasm(&[0x00, 0x1e]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Bl),
            Addr::Indirect(Indirect::Base(Reg::Esi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x1f]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Bl),
            Addr::Indirect(Indirect::Base(Reg::Edi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x20]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Ah),
            Addr::Indirect(Indirect::Base(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x21]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Ah),
            Addr::Indirect(Indirect::Base(Reg::Ecx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x22]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Ah),
            Addr::Indirect(Indirect::Base(Reg::Edx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x23]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Ah),
            Addr::Indirect(Indirect::Base(Reg::Ebx)),
        )])
    );
    assert_eq!(Dasha::disasm(&[0x00, 0x24]), Err(Error::ExpectedSib));
    assert_eq!(Dasha::disasm(&[0x00, 0x25]), Err(Error::ExpectedOffsetLong));
    assert_eq!(
        Dasha::disasm(&[0x00, 0x26]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Ah),
            Addr::Indirect(Indirect::Base(Reg::Esi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x27]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Ah),
            Addr::Indirect(Indirect::Base(Reg::Edi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x28]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Ch),
            Addr::Indirect(Indirect::Base(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x29]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Ch),
            Addr::Indirect(Indirect::Base(Reg::Ecx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x2a]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Ch),
            Addr::Indirect(Indirect::Base(Reg::Edx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x2b]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Ch),
            Addr::Indirect(Indirect::Base(Reg::Ebx)),
        )])
    );
    assert_eq!(Dasha::disasm(&[0x00, 0x2c]), Err(Error::ExpectedSib));
    assert_eq!(Dasha::disasm(&[0x00, 0x2d]), Err(Error::ExpectedOffsetLong));
    assert_eq!(
        Dasha::disasm(&[0x00, 0x2e]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Ch),
            Addr::Indirect(Indirect::Base(Reg::Esi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x2f]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Ch),
            Addr::Indirect(Indirect::Base(Reg::Edi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x30]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Dh),
            Addr::Indirect(Indirect::Base(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x31]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Dh),
            Addr::Indirect(Indirect::Base(Reg::Ecx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x32]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Dh),
            Addr::Indirect(Indirect::Base(Reg::Edx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x33]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Dh),
            Addr::Indirect(Indirect::Base(Reg::Ebx)),
        )])
    );
    assert_eq!(Dasha::disasm(&[0x00, 0x34]), Err(Error::ExpectedSib));
    assert_eq!(Dasha::disasm(&[0x00, 0x35]), Err(Error::ExpectedOffsetLong));
    assert_eq!(
        Dasha::disasm(&[0x00, 0x36]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Dh),
            Addr::Indirect(Indirect::Base(Reg::Esi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x37]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Dh),
            Addr::Indirect(Indirect::Base(Reg::Edi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x38]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Bh),
            Addr::Indirect(Indirect::Base(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x39]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Bh),
            Addr::Indirect(Indirect::Base(Reg::Ecx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x3a]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Bh),
            Addr::Indirect(Indirect::Base(Reg::Edx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x3b]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Bh),
            Addr::Indirect(Indirect::Base(Reg::Ebx)),
        )])
    );
    assert_eq!(Dasha::disasm(&[0x00, 0x3c]), Err(Error::ExpectedSib));
    assert_eq!(Dasha::disasm(&[0x00, 0x3d]), Err(Error::ExpectedOffsetLong));
    assert_eq!(
        Dasha::disasm(&[0x00, 0x3e]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Bh),
            Addr::Indirect(Indirect::Base(Reg::Esi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x3f]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Bh),
            Addr::Indirect(Indirect::Base(Reg::Edi)),
        )])
    );
}

#[test]
fn test_indirect_mem_addressing() {
    assert_eq!(
        Dasha::disasm(&[0x00, 0x05, 0x00, 0x00, 0x00, 0x00]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Al),
            Addr::Indirect(Indirect::Mem(Size::Byte, 0)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x05, 0xff, 0xff, 0xff, 0x7f]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Al),
            Addr::Indirect(Indirect::Mem(Size::Byte, 0x7fffffff)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x05, 0x00, 0x00, 0x00, 0x80]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Al),
            Addr::Indirect(Indirect::Mem(Size::Byte, -0x80000000)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x05, 0xff, 0xff, 0xff, 0xff]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Al),
            Addr::Indirect(Indirect::Mem(Size::Byte, -1)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x0d, 0x00, 0x00, 0x00, 0x00]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Cl),
            Addr::Indirect(Indirect::Mem(Size::Byte, 0)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x0d, 0xff, 0xff, 0xff, 0x7f]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Cl),
            Addr::Indirect(Indirect::Mem(Size::Byte, 0x7fffffff)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x0d, 0x00, 0x00, 0x00, 0x80]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Cl),
            Addr::Indirect(Indirect::Mem(Size::Byte, -0x80000000)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x0d, 0xff, 0xff, 0xff, 0xff]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Cl),
            Addr::Indirect(Indirect::Mem(Size::Byte, -1)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x15, 0x00, 0x00, 0x00, 0x00]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Dl),
            Addr::Indirect(Indirect::Mem(Size::Byte, 0)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x15, 0xff, 0xff, 0xff, 0x7f]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Dl),
            Addr::Indirect(Indirect::Mem(Size::Byte, 0x7fffffff)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x15, 0x00, 0x00, 0x00, 0x80]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Dl),
            Addr::Indirect(Indirect::Mem(Size::Byte, -0x80000000)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x15, 0xff, 0xff, 0xff, 0xff]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Dl),
            Addr::Indirect(Indirect::Mem(Size::Byte, -1)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x1d, 0x00, 0x00, 0x00, 0x00]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Bl),
            Addr::Indirect(Indirect::Mem(Size::Byte, 0)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x1d, 0xff, 0xff, 0xff, 0x7f]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Bl),
            Addr::Indirect(Indirect::Mem(Size::Byte, 0x7fffffff)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x1d, 0x00, 0x00, 0x00, 0x80]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Bl),
            Addr::Indirect(Indirect::Mem(Size::Byte, -0x80000000)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x1d, 0xff, 0xff, 0xff, 0xff]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Bl),
            Addr::Indirect(Indirect::Mem(Size::Byte, -1)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x25, 0x00, 0x00, 0x00, 0x00]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Ah),
            Addr::Indirect(Indirect::Mem(Size::Byte, 0)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x25, 0xff, 0xff, 0xff, 0x7f]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Ah),
            Addr::Indirect(Indirect::Mem(Size::Byte, 0x7fffffff)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x25, 0x00, 0x00, 0x00, 0x80]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Ah),
            Addr::Indirect(Indirect::Mem(Size::Byte, -0x80000000)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x25, 0xff, 0xff, 0xff, 0xff]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Ah),
            Addr::Indirect(Indirect::Mem(Size::Byte, -1)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x2d, 0x00, 0x00, 0x00, 0x00]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Ch),
            Addr::Indirect(Indirect::Mem(Size::Byte, 0)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x2d, 0xff, 0xff, 0xff, 0x7f]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Ch),
            Addr::Indirect(Indirect::Mem(Size::Byte, 0x7fffffff)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x2d, 0x00, 0x00, 0x00, 0x80]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Ch),
            Addr::Indirect(Indirect::Mem(Size::Byte, -0x80000000)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x2d, 0xff, 0xff, 0xff, 0xff]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Ch),
            Addr::Indirect(Indirect::Mem(Size::Byte, -1)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x35, 0x00, 0x00, 0x00, 0x00]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Dh),
            Addr::Indirect(Indirect::Mem(Size::Byte, 0)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x35, 0xff, 0xff, 0xff, 0x7f]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Dh),
            Addr::Indirect(Indirect::Mem(Size::Byte, 0x7fffffff)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x35, 0x00, 0x00, 0x00, 0x80]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Dh),
            Addr::Indirect(Indirect::Mem(Size::Byte, -0x80000000)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x35, 0xff, 0xff, 0xff, 0xff]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Dh),
            Addr::Indirect(Indirect::Mem(Size::Byte, -1)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x3d, 0x00, 0x00, 0x00, 0x00]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Bh),
            Addr::Indirect(Indirect::Mem(Size::Byte, 0)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x3d, 0xff, 0xff, 0xff, 0x7f]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Bh),
            Addr::Indirect(Indirect::Mem(Size::Byte, 0x7fffffff)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x3d, 0x00, 0x00, 0x00, 0x80]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Bh),
            Addr::Indirect(Indirect::Mem(Size::Byte, -0x80000000)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x3d, 0xff, 0xff, 0xff, 0xff]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Bh),
            Addr::Indirect(Indirect::Mem(Size::Byte, -1)),
        )])
    );
}
