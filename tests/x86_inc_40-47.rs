use dasha::{Addr, Dasha, Inst, Reg, Val};

#[test]
fn test_inst_40() {
    assert_eq!(
        Dasha::disasm(&[0x40]),
        Ok(vec![Inst::Inc(Val::Addr(Addr::Direct(Reg::Eax)))])
    );
}

#[test]
fn test_inst_41() {
    assert_eq!(
        Dasha::disasm(&[0x41]),
        Ok(vec![Inst::Inc(Val::Addr(Addr::Direct(Reg::Ecx)))])
    );
}

#[test]
fn test_inst_42() {
    assert_eq!(
        Dasha::disasm(&[0x42]),
        Ok(vec![Inst::Inc(Val::Addr(Addr::Direct(Reg::Edx)))])
    );
}

#[test]
fn test_inst_43() {
    assert_eq!(
        Dasha::disasm(&[0x43]),
        Ok(vec![Inst::Inc(Val::Addr(Addr::Direct(Reg::Ebx)))])
    );
}

#[test]
fn test_inst_44() {
    assert_eq!(
        Dasha::disasm(&[0x44]),
        Ok(vec![Inst::Inc(Val::Addr(Addr::Direct(Reg::Esp)))])
    );
}

#[test]
fn test_inst_45() {
    assert_eq!(
        Dasha::disasm(&[0x45]),
        Ok(vec![Inst::Inc(Val::Addr(Addr::Direct(Reg::Ebp)))])
    );
}

#[test]
fn test_inst_46() {
    assert_eq!(
        Dasha::disasm(&[0x46]),
        Ok(vec![Inst::Inc(Val::Addr(Addr::Direct(Reg::Esi)))])
    );
}

#[test]
fn test_inst_47() {
    assert_eq!(
        Dasha::disasm(&[0x47]),
        Ok(vec![Inst::Inc(Val::Addr(Addr::Direct(Reg::Edi)))])
    );
}
