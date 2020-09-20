use dasha::{Dasha, Inst};

#[test]
fn test_imm_val() {
    assert_eq!(Dasha::disasm(&[0x61]), Ok(vec![Inst::PopA]));
}
