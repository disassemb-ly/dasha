use dasha::{Addr, Dasha, Inst, Reg, Val};

#[test]
fn test_inst_48() {
    assert_eq!(
        Dasha::disasm(&[0x48]),
        Ok(vec![Inst::Dec(Val::Addr(Addr::Direct(Reg::Eax)))])
    );
}

#[test]
fn test_inst_49() {
    assert_eq!(
        Dasha::disasm(&[0x49]),
        Ok(vec![Inst::Dec(Val::Addr(Addr::Direct(Reg::Ecx)))])
    );
}

#[test]
fn test_inst_4a() {
    assert_eq!(
        Dasha::disasm(&[0x4a]),
        Ok(vec![Inst::Dec(Val::Addr(Addr::Direct(Reg::Edx)))])
    );
}

#[test]
fn test_inst_4b() {
    assert_eq!(
        Dasha::disasm(&[0x4b]),
        Ok(vec![Inst::Dec(Val::Addr(Addr::Direct(Reg::Ebx)))])
    );
}

#[test]
fn test_inst_4c() {
    assert_eq!(
        Dasha::disasm(&[0x4c]),
        Ok(vec![Inst::Dec(Val::Addr(Addr::Direct(Reg::Esp)))])
    );
}

#[test]
fn test_inst_4d() {
    assert_eq!(
        Dasha::disasm(&[0x4d]),
        Ok(vec![Inst::Dec(Val::Addr(Addr::Direct(Reg::Ebp)))])
    );
}

#[test]
fn test_inst_4e() {
    assert_eq!(
        Dasha::disasm(&[0x4e]),
        Ok(vec![Inst::Dec(Val::Addr(Addr::Direct(Reg::Esi)))])
    );
}

#[test]
fn test_inst_4f() {
    assert_eq!(
        Dasha::disasm(&[0x4f]),
        Ok(vec![Inst::Dec(Val::Addr(Addr::Direct(Reg::Edi)))])
    );
}
