use dasha::{Dasha, Inst};

#[test]
fn test_inst() {
    assert_eq!(Dasha::disasm(&[0x2f]), Ok(vec![Inst::Das]));
}
