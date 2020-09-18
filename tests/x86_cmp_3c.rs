use dasha::{Addr, Dasha, Error, Imm, Inst, Reg, Val};

#[test]
fn test_imm_val() {
    assert_eq!(Dasha::disasm(&[0x3c]), Err(Error::ExpectedImmByte));
    assert_eq!(
        Dasha::disasm(&[0x3c, 0x00]),
        Ok(vec![Inst::Cmp(
            Val::Addr(Addr::Direct(Reg::Al)),
            Val::Imm(Imm::U8(0))
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x3c, 0x7f]),
        Ok(vec![Inst::Cmp(
            Val::Addr(Addr::Direct(Reg::Al)),
            Val::Imm(Imm::U8(0x7f))
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x3c, 0x80]),
        Ok(vec![Inst::Cmp(
            Val::Addr(Addr::Direct(Reg::Al)),
            Val::Imm(Imm::U8(0x80))
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x3c, 0xff]),
        Ok(vec![Inst::Cmp(
            Val::Addr(Addr::Direct(Reg::Al)),
            Val::Imm(Imm::U8(0xff))
        )])
    );
}
