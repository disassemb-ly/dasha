use dasha::{Addr, Dasha, Inst, Reg, Val};

#[test]
fn test_inst_50() {
    assert_eq!(
        Dasha::disasm(&[0x50]),
        Ok(vec![Inst::Push(Val::Addr(Addr::Direct(Reg::Eax)))])
    );
}

#[test]
fn test_inst_51() {
    assert_eq!(
        Dasha::disasm(&[0x51]),
        Ok(vec![Inst::Push(Val::Addr(Addr::Direct(Reg::Ecx)))])
    );
}

#[test]
fn test_inst_52() {
    assert_eq!(
        Dasha::disasm(&[0x52]),
        Ok(vec![Inst::Push(Val::Addr(Addr::Direct(Reg::Edx)))])
    );
}

#[test]
fn test_inst_53() {
    assert_eq!(
        Dasha::disasm(&[0x53]),
        Ok(vec![Inst::Push(Val::Addr(Addr::Direct(Reg::Ebx)))])
    );
}

#[test]
fn test_inst_54() {
    assert_eq!(
        Dasha::disasm(&[0x54]),
        Ok(vec![Inst::Push(Val::Addr(Addr::Direct(Reg::Esp)))])
    );
}

#[test]
fn test_inst_55() {
    assert_eq!(
        Dasha::disasm(&[0x55]),
        Ok(vec![Inst::Push(Val::Addr(Addr::Direct(Reg::Ebp)))])
    );
}

#[test]
fn test_inst_56() {
    assert_eq!(
        Dasha::disasm(&[0x56]),
        Ok(vec![Inst::Push(Val::Addr(Addr::Direct(Reg::Esi)))])
    );
}

#[test]
fn test_inst_57() {
    assert_eq!(
        Dasha::disasm(&[0x57]),
        Ok(vec![Inst::Push(Val::Addr(Addr::Direct(Reg::Edi)))])
    );
}
