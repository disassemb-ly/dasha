use dasha::{Addr, Dasha, Inst, Reg};

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
