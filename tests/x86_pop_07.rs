use dasha::{Addr, Dasha, Inst, Reg, Val};

#[test]
fn test_inst() {
    assert_eq!(
        Dasha::disasm(&[0x07]),
        Ok(vec![Inst::Pop(Val::Addr(Addr::Direct(Reg::Es)))])
    );
}
