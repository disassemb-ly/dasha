use dasha::{Addr, Dasha, Error, Indirect, Inst, Reg};

#[test]
fn test_indirect_addressing() {
    assert_eq!(
        Dasha::disasm(&[0x00, 0x00]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Al),
            Addr::Indirect(Indirect::Base(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x01]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Al),
            Addr::Indirect(Indirect::Base(Reg::Ecx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x02]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Al),
            Addr::Indirect(Indirect::Base(Reg::Edx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x03]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Al),
            Addr::Indirect(Indirect::Base(Reg::Ebx)),
        )])
    );
    assert_eq!(Dasha::disasm(&[0x00, 0x04]), Err(Error::ExpectedSib));
    assert_eq!(Dasha::disasm(&[0x00, 0x05]), Err(Error::ExpectedOffsetLong));
    assert_eq!(
        Dasha::disasm(&[0x00, 0x06]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Al),
            Addr::Indirect(Indirect::Base(Reg::Esi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x07]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Al),
            Addr::Indirect(Indirect::Base(Reg::Edi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x08]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Cl),
            Addr::Indirect(Indirect::Base(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x09]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Cl),
            Addr::Indirect(Indirect::Base(Reg::Ecx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x0a]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Cl),
            Addr::Indirect(Indirect::Base(Reg::Edx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x0b]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Cl),
            Addr::Indirect(Indirect::Base(Reg::Ebx)),
        )])
    );
    assert_eq!(Dasha::disasm(&[0x00, 0x0c]), Err(Error::ExpectedSib));
    assert_eq!(Dasha::disasm(&[0x00, 0x0d]), Err(Error::ExpectedOffsetLong));
    assert_eq!(
        Dasha::disasm(&[0x00, 0x0e]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Cl),
            Addr::Indirect(Indirect::Base(Reg::Esi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x0f]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Cl),
            Addr::Indirect(Indirect::Base(Reg::Edi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x10]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Dl),
            Addr::Indirect(Indirect::Base(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x11]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Dl),
            Addr::Indirect(Indirect::Base(Reg::Ecx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x12]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Dl),
            Addr::Indirect(Indirect::Base(Reg::Edx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x13]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Dl),
            Addr::Indirect(Indirect::Base(Reg::Ebx)),
        )])
    );
    assert_eq!(Dasha::disasm(&[0x00, 0x14]), Err(Error::ExpectedSib));
    assert_eq!(Dasha::disasm(&[0x00, 0x15]), Err(Error::ExpectedOffsetLong));
    assert_eq!(
        Dasha::disasm(&[0x00, 0x16]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Dl),
            Addr::Indirect(Indirect::Base(Reg::Esi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x17]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Dl),
            Addr::Indirect(Indirect::Base(Reg::Edi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x18]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Bl),
            Addr::Indirect(Indirect::Base(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x19]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Bl),
            Addr::Indirect(Indirect::Base(Reg::Ecx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x1a]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Bl),
            Addr::Indirect(Indirect::Base(Reg::Edx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x1b]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Bl),
            Addr::Indirect(Indirect::Base(Reg::Ebx)),
        )])
    );
    assert_eq!(Dasha::disasm(&[0x00, 0x1c]), Err(Error::ExpectedSib));
    assert_eq!(Dasha::disasm(&[0x00, 0x1d]), Err(Error::ExpectedOffsetLong));
    assert_eq!(
        Dasha::disasm(&[0x00, 0x1e]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Bl),
            Addr::Indirect(Indirect::Base(Reg::Esi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x1f]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Bl),
            Addr::Indirect(Indirect::Base(Reg::Edi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x20]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Ah),
            Addr::Indirect(Indirect::Base(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x21]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Ah),
            Addr::Indirect(Indirect::Base(Reg::Ecx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x22]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Ah),
            Addr::Indirect(Indirect::Base(Reg::Edx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x23]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Ah),
            Addr::Indirect(Indirect::Base(Reg::Ebx)),
        )])
    );
    assert_eq!(Dasha::disasm(&[0x00, 0x24]), Err(Error::ExpectedSib));
    assert_eq!(Dasha::disasm(&[0x00, 0x25]), Err(Error::ExpectedOffsetLong));
    assert_eq!(
        Dasha::disasm(&[0x00, 0x26]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Ah),
            Addr::Indirect(Indirect::Base(Reg::Esi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x27]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Ah),
            Addr::Indirect(Indirect::Base(Reg::Edi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x28]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Ch),
            Addr::Indirect(Indirect::Base(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x29]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Ch),
            Addr::Indirect(Indirect::Base(Reg::Ecx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x2a]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Ch),
            Addr::Indirect(Indirect::Base(Reg::Edx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x2b]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Ch),
            Addr::Indirect(Indirect::Base(Reg::Ebx)),
        )])
    );
    assert_eq!(Dasha::disasm(&[0x00, 0x2c]), Err(Error::ExpectedSib));
    assert_eq!(Dasha::disasm(&[0x00, 0x2d]), Err(Error::ExpectedOffsetLong));
    assert_eq!(
        Dasha::disasm(&[0x00, 0x2e]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Ch),
            Addr::Indirect(Indirect::Base(Reg::Esi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x2f]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Ch),
            Addr::Indirect(Indirect::Base(Reg::Edi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x30]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Dh),
            Addr::Indirect(Indirect::Base(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x31]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Dh),
            Addr::Indirect(Indirect::Base(Reg::Ecx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x32]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Dh),
            Addr::Indirect(Indirect::Base(Reg::Edx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x33]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Dh),
            Addr::Indirect(Indirect::Base(Reg::Ebx)),
        )])
    );
    assert_eq!(Dasha::disasm(&[0x00, 0x34]), Err(Error::ExpectedSib));
    assert_eq!(Dasha::disasm(&[0x00, 0x35]), Err(Error::ExpectedOffsetLong));
    assert_eq!(
        Dasha::disasm(&[0x00, 0x36]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Dh),
            Addr::Indirect(Indirect::Base(Reg::Esi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x37]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Dh),
            Addr::Indirect(Indirect::Base(Reg::Edi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x38]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Bh),
            Addr::Indirect(Indirect::Base(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x39]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Bh),
            Addr::Indirect(Indirect::Base(Reg::Ecx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x3a]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Bh),
            Addr::Indirect(Indirect::Base(Reg::Edx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x3b]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Bh),
            Addr::Indirect(Indirect::Base(Reg::Ebx)),
        )])
    );
    assert_eq!(Dasha::disasm(&[0x00, 0x3c]), Err(Error::ExpectedSib));
    assert_eq!(Dasha::disasm(&[0x00, 0x3d]), Err(Error::ExpectedOffsetLong));
    assert_eq!(
        Dasha::disasm(&[0x00, 0x3e]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Bh),
            Addr::Indirect(Indirect::Base(Reg::Esi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x00, 0x3f]),
        Ok(vec![Inst::Add(
            Addr::Direct(Reg::Bh),
            Addr::Indirect(Indirect::Base(Reg::Edi)),
        )])
    );
}
