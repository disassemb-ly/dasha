use dasha::{Addr, Dasha, Inst, Reg, Val};

#[test]
fn test_inst_58() {
    assert_eq!(
        Dasha::disasm(&[0x58]),
        Ok(vec![Inst::Pop(Val::Addr(Addr::Direct(Reg::Eax)))])
    );
}

#[test]
fn test_inst_59() {
    assert_eq!(
        Dasha::disasm(&[0x59]),
        Ok(vec![Inst::Pop(Val::Addr(Addr::Direct(Reg::Ecx)))])
    );
}

#[test]
fn test_inst_5a() {
    assert_eq!(
        Dasha::disasm(&[0x5a]),
        Ok(vec![Inst::Pop(Val::Addr(Addr::Direct(Reg::Edx)))])
    );
}

#[test]
fn test_inst_5b() {
    assert_eq!(
        Dasha::disasm(&[0x5b]),
        Ok(vec![Inst::Pop(Val::Addr(Addr::Direct(Reg::Ebx)))])
    );
}

#[test]
fn test_inst_5c() {
    assert_eq!(
        Dasha::disasm(&[0x5c]),
        Ok(vec![Inst::Pop(Val::Addr(Addr::Direct(Reg::Esp)))])
    );
}

#[test]
fn test_inst_5d() {
    assert_eq!(
        Dasha::disasm(&[0x5d]),
        Ok(vec![Inst::Pop(Val::Addr(Addr::Direct(Reg::Ebp)))])
    );
}

#[test]
fn test_inst_5e() {
    assert_eq!(
        Dasha::disasm(&[0x5e]),
        Ok(vec![Inst::Pop(Val::Addr(Addr::Direct(Reg::Esi)))])
    );
}

#[test]
fn test_inst_5f() {
    assert_eq!(
        Dasha::disasm(&[0x5f]),
        Ok(vec![Inst::Pop(Val::Addr(Addr::Direct(Reg::Edi)))])
    );
}
