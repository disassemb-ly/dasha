use dasha::{Dasha, Inst};

#[test]
fn test_inst() {
    assert_eq!(Dasha::disasm(&[0x27]), Ok(vec![Inst::Daa]));
}
