use dasha::{Addr, Dasha, Inst, Reg, Val};

#[test]
fn test_inst() {
    assert_eq!(
        Dasha::disasm(&[0x06]),
        Ok(vec![Inst::Push(Val::Addr(Addr::Direct(Reg::Es)))])
    );
}
