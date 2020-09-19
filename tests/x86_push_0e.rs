use dasha::{Addr, Dasha, Inst, Reg, Val};

#[test]
fn test_inst() {
    assert_eq!(
        Dasha::disasm(&[0x0e]),
        Ok(vec![Inst::Push(Val::Addr(Addr::Direct(Reg::Cs)))])
    );
}
