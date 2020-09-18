use dasha::{Addr, Dasha, Error, Imm, Inst, Reg, Val};

#[test]
fn test_imm_val() {
    assert_eq!(Dasha::disasm(&[0x04]), Err(Error::ExpectedImmByte));
    assert_eq!(
        Dasha::disasm(&[0x14, 0x00]),
        Ok(vec![Inst::Adc(
            Val::Addr(Addr::Direct(Reg::Al)),
            Val::Imm(Imm::U8(0))
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x14, 0x7f]),
        Ok(vec![Inst::Adc(
            Val::Addr(Addr::Direct(Reg::Al)),
            Val::Imm(Imm::U8(0x7f))
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x14, 0x80]),
        Ok(vec![Inst::Adc(
            Val::Addr(Addr::Direct(Reg::Al)),
            Val::Imm(Imm::U8(0x80))
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x14, 0xff]),
        Ok(vec![Inst::Adc(
            Val::Addr(Addr::Direct(Reg::Al)),
            Val::Imm(Imm::U8(0xff))
        )])
    );
}
