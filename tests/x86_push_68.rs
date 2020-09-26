use dasha::{Dasha, Error, Imm, Inst, Val};

#[test]
fn test_inst() {
    assert_eq!(Dasha::disasm(&[0x68]), Err(Error::ExpectedImmLong));
    assert_eq!(Dasha::disasm(&[0x68, 0x00]), Err(Error::ExpectedImmLong));
    assert_eq!(
        Dasha::disasm(&[0x68, 0x00, 0x00]),
        Err(Error::ExpectedImmLong)
    );
    assert_eq!(
        Dasha::disasm(&[0x68, 0x00, 0x00, 0x00]),
        Err(Error::ExpectedImmLong)
    );
    assert_eq!(
        Dasha::disasm(&[0x68, 0x00, 0x00, 0x00, 0x00]),
        Ok(vec![Inst::Push(Val::Imm(Imm::U32(0)))])
    );
    assert_eq!(
        Dasha::disasm(&[0x68, 0xff, 0xff, 0xff, 0x7f]),
        Ok(vec![Inst::Push(Val::Imm(Imm::U32(0x7fffffff)))])
    );
    assert_eq!(
        Dasha::disasm(&[0x68, 0x00, 0x00, 0x00, 0x80]),
        Ok(vec![Inst::Push(Val::Imm(Imm::U32(0x80000000)))])
    );
    assert_eq!(
        Dasha::disasm(&[0x68, 0xff, 0xff, 0xff, 0xff]),
        Ok(vec![Inst::Push(Val::Imm(Imm::U32(0xffffffff)))])
    );
}
