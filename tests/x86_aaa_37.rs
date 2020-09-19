use dasha::{Dasha, Inst};

#[test]
fn test_inst() {
    assert_eq!(Dasha::disasm(&[0x37]), Ok(vec![Inst::Aaa]));
}
