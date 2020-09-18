use dasha::{Addr, Dasha, Error, Imm, Inst, Reg, Val};

#[test]
fn test_imm_val() {
    assert_eq!(Dasha::disasm(&[0x25]), Err(Error::ExpectedImmLong));
    assert_eq!(Dasha::disasm(&[0x25, 0x00]), Err(Error::ExpectedImmLong));
    assert_eq!(
        Dasha::disasm(&[0x25, 0x00, 0x00]),
        Err(Error::ExpectedImmLong)
    );
    assert_eq!(
        Dasha::disasm(&[0x25, 0x00, 0x00, 0x00]),
        Err(Error::ExpectedImmLong)
    );
    assert_eq!(
        Dasha::disasm(&[0x25, 0x00, 0x00, 0x00, 0x00]),
        Ok(vec![Inst::And(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Imm(Imm::U32(0))
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x25, 0xff, 0xff, 0xff, 0x7f]),
        Ok(vec![Inst::And(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Imm(Imm::U32(0x7fffffff))
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x25, 0x00, 0x00, 0x00, 0x80]),
        Ok(vec![Inst::And(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Imm(Imm::U32(0x80000000))
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x25, 0xff, 0xff, 0xff, 0xff]),
        Ok(vec![Inst::And(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Imm(Imm::U32(0xffffffff))
        )])
    );
}
