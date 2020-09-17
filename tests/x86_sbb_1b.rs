use dasha::{Addr, Dasha, Error, Indirect, Inst, Offset, Reg, Scale, Size, Val};

#[test]
fn test_direct_long_addressing() {
    assert_eq!(
        Dasha::disasm(&[0x1b, 0xc0]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0xc1]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Direct(Reg::Ecx)),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0xc2]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Direct(Reg::Edx)),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0xc3]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Direct(Reg::Ebx)),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0xc4]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Direct(Reg::Esp)),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0xc5]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Direct(Reg::Ebp)),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0xc6]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Direct(Reg::Esi)),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0xc7]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Direct(Reg::Edi)),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0xc8]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Direct(Reg::Ecx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0xc9]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Direct(Reg::Ecx)),
            Val::Addr(Addr::Direct(Reg::Ecx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0xca]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Direct(Reg::Edx)),
            Val::Addr(Addr::Direct(Reg::Ecx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0xcb]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Direct(Reg::Ebx)),
            Val::Addr(Addr::Direct(Reg::Ecx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0xcc]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Direct(Reg::Esp)),
            Val::Addr(Addr::Direct(Reg::Ecx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0xcd]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Direct(Reg::Ebp)),
            Val::Addr(Addr::Direct(Reg::Ecx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0xce]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Direct(Reg::Esi)),
            Val::Addr(Addr::Direct(Reg::Ecx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0xcf]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Direct(Reg::Edi)),
            Val::Addr(Addr::Direct(Reg::Ecx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0xd0]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Direct(Reg::Edx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0xd1]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Direct(Reg::Ecx)),
            Val::Addr(Addr::Direct(Reg::Edx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0xd2]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Direct(Reg::Edx)),
            Val::Addr(Addr::Direct(Reg::Edx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0xd3]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Direct(Reg::Ebx)),
            Val::Addr(Addr::Direct(Reg::Edx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0xd4]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Direct(Reg::Esp)),
            Val::Addr(Addr::Direct(Reg::Edx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0xd5]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Direct(Reg::Ebp)),
            Val::Addr(Addr::Direct(Reg::Edx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0xd6]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Direct(Reg::Esi)),
            Val::Addr(Addr::Direct(Reg::Edx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0xd7]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Direct(Reg::Edi)),
            Val::Addr(Addr::Direct(Reg::Edx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0xd8]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Direct(Reg::Ebx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0xd9]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Direct(Reg::Ecx)),
            Val::Addr(Addr::Direct(Reg::Ebx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0xda]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Direct(Reg::Edx)),
            Val::Addr(Addr::Direct(Reg::Ebx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0xdb]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Direct(Reg::Ebx)),
            Val::Addr(Addr::Direct(Reg::Ebx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0xdc]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Direct(Reg::Esp)),
            Val::Addr(Addr::Direct(Reg::Ebx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0xdd]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Direct(Reg::Ebp)),
            Val::Addr(Addr::Direct(Reg::Ebx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0xde]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Direct(Reg::Esi)),
            Val::Addr(Addr::Direct(Reg::Ebx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0xdf]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Direct(Reg::Edi)),
            Val::Addr(Addr::Direct(Reg::Ebx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0xe0]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Direct(Reg::Esp)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0xe1]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Direct(Reg::Ecx)),
            Val::Addr(Addr::Direct(Reg::Esp)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0xe2]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Direct(Reg::Edx)),
            Val::Addr(Addr::Direct(Reg::Esp)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0xe3]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Direct(Reg::Ebx)),
            Val::Addr(Addr::Direct(Reg::Esp)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0xe4]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Direct(Reg::Esp)),
            Val::Addr(Addr::Direct(Reg::Esp)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0xe5]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Direct(Reg::Ebp)),
            Val::Addr(Addr::Direct(Reg::Esp)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0xe6]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Direct(Reg::Esi)),
            Val::Addr(Addr::Direct(Reg::Esp)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0xe7]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Direct(Reg::Edi)),
            Val::Addr(Addr::Direct(Reg::Esp)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0xe8]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Direct(Reg::Ebp)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0xe9]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Direct(Reg::Ecx)),
            Val::Addr(Addr::Direct(Reg::Ebp)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0xea]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Direct(Reg::Edx)),
            Val::Addr(Addr::Direct(Reg::Ebp)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0xeb]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Direct(Reg::Ebx)),
            Val::Addr(Addr::Direct(Reg::Ebp)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0xec]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Direct(Reg::Esp)),
            Val::Addr(Addr::Direct(Reg::Ebp)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0xed]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Direct(Reg::Ebp)),
            Val::Addr(Addr::Direct(Reg::Ebp)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0xee]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Direct(Reg::Esi)),
            Val::Addr(Addr::Direct(Reg::Ebp)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0xef]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Direct(Reg::Edi)),
            Val::Addr(Addr::Direct(Reg::Ebp)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0xf0]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Direct(Reg::Esi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0xf1]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Direct(Reg::Ecx)),
            Val::Addr(Addr::Direct(Reg::Esi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0xf2]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Direct(Reg::Edx)),
            Val::Addr(Addr::Direct(Reg::Esi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0xf3]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Direct(Reg::Ebx)),
            Val::Addr(Addr::Direct(Reg::Esi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0xf4]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Direct(Reg::Esp)),
            Val::Addr(Addr::Direct(Reg::Esi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0xf5]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Direct(Reg::Ebp)),
            Val::Addr(Addr::Direct(Reg::Esi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0xf6]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Direct(Reg::Esi)),
            Val::Addr(Addr::Direct(Reg::Esi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0xf7]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Direct(Reg::Edi)),
            Val::Addr(Addr::Direct(Reg::Esi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0xf8]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Direct(Reg::Edi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0xf9]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Direct(Reg::Ecx)),
            Val::Addr(Addr::Direct(Reg::Edi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0xfa]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Direct(Reg::Edx)),
            Val::Addr(Addr::Direct(Reg::Edi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0xfb]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Direct(Reg::Ebx)),
            Val::Addr(Addr::Direct(Reg::Edi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0xfc]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Direct(Reg::Esp)),
            Val::Addr(Addr::Direct(Reg::Edi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0xfd]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Direct(Reg::Ebp)),
            Val::Addr(Addr::Direct(Reg::Edi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0xfe]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Direct(Reg::Esi)),
            Val::Addr(Addr::Direct(Reg::Edi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0xff]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Direct(Reg::Edi)),
            Val::Addr(Addr::Direct(Reg::Edi)),
        )])
    );
}

#[test]
fn test_indirect_base_long_addressing() {
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x00]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Eax))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x01]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Ecx))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x02]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Edx))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x03]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Ebx))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(Dasha::disasm(&[0x1b, 0x04]), Err(Error::ExpectedSib));
    assert_eq!(Dasha::disasm(&[0x1b, 0x05]), Err(Error::ExpectedOffsetLong));
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x06]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Esi))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x07]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Edi))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x08]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Eax))),
            Val::Addr(Addr::Direct(Reg::Ecx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x09]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Ecx))),
            Val::Addr(Addr::Direct(Reg::Ecx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x0a]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Edx))),
            Val::Addr(Addr::Direct(Reg::Ecx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x0b]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Ebx))),
            Val::Addr(Addr::Direct(Reg::Ecx)),
        )])
    );
    assert_eq!(Dasha::disasm(&[0x1b, 0x0c]), Err(Error::ExpectedSib));
    assert_eq!(Dasha::disasm(&[0x1b, 0x0d]), Err(Error::ExpectedOffsetLong));
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x0e]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Esi))),
            Val::Addr(Addr::Direct(Reg::Ecx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x0f]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Edi))),
            Val::Addr(Addr::Direct(Reg::Ecx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x10]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Eax))),
            Val::Addr(Addr::Direct(Reg::Edx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x11]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Ecx))),
            Val::Addr(Addr::Direct(Reg::Edx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x12]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Edx))),
            Val::Addr(Addr::Direct(Reg::Edx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x13]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Ebx))),
            Val::Addr(Addr::Direct(Reg::Edx)),
        )])
    );
    assert_eq!(Dasha::disasm(&[0x1b, 0x14]), Err(Error::ExpectedSib));
    assert_eq!(Dasha::disasm(&[0x1b, 0x15]), Err(Error::ExpectedOffsetLong));
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x16]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Esi))),
            Val::Addr(Addr::Direct(Reg::Edx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x17]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Edi))),
            Val::Addr(Addr::Direct(Reg::Edx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x18]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Eax))),
            Val::Addr(Addr::Direct(Reg::Ebx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x19]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Ecx))),
            Val::Addr(Addr::Direct(Reg::Ebx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x1a]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Edx))),
            Val::Addr(Addr::Direct(Reg::Ebx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x1b]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Ebx))),
            Val::Addr(Addr::Direct(Reg::Ebx)),
        )])
    );
    assert_eq!(Dasha::disasm(&[0x1b, 0x1c]), Err(Error::ExpectedSib));
    assert_eq!(Dasha::disasm(&[0x1b, 0x1d]), Err(Error::ExpectedOffsetLong));
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x1e]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Esi))),
            Val::Addr(Addr::Direct(Reg::Ebx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x1f]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Edi))),
            Val::Addr(Addr::Direct(Reg::Ebx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x20]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Eax))),
            Val::Addr(Addr::Direct(Reg::Esp)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x21]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Ecx))),
            Val::Addr(Addr::Direct(Reg::Esp)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x22]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Edx))),
            Val::Addr(Addr::Direct(Reg::Esp)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x23]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Ebx))),
            Val::Addr(Addr::Direct(Reg::Esp)),
        )])
    );
    assert_eq!(Dasha::disasm(&[0x1b, 0x24]), Err(Error::ExpectedSib));
    assert_eq!(Dasha::disasm(&[0x1b, 0x25]), Err(Error::ExpectedOffsetLong));
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x26]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Esi))),
            Val::Addr(Addr::Direct(Reg::Esp)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x27]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Edi))),
            Val::Addr(Addr::Direct(Reg::Esp)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x28]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Eax))),
            Val::Addr(Addr::Direct(Reg::Ebp)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x29]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Ecx))),
            Val::Addr(Addr::Direct(Reg::Ebp)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x2a]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Edx))),
            Val::Addr(Addr::Direct(Reg::Ebp)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x2b]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Ebx))),
            Val::Addr(Addr::Direct(Reg::Ebp)),
        )])
    );
    assert_eq!(Dasha::disasm(&[0x1b, 0x2c]), Err(Error::ExpectedSib));
    assert_eq!(Dasha::disasm(&[0x1b, 0x2d]), Err(Error::ExpectedOffsetLong));
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x2e]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Esi))),
            Val::Addr(Addr::Direct(Reg::Ebp)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x2f]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Edi))),
            Val::Addr(Addr::Direct(Reg::Ebp)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x30]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Eax))),
            Val::Addr(Addr::Direct(Reg::Esi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x31]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Ecx))),
            Val::Addr(Addr::Direct(Reg::Esi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x32]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Edx))),
            Val::Addr(Addr::Direct(Reg::Esi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x33]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Ebx))),
            Val::Addr(Addr::Direct(Reg::Esi)),
        )])
    );
    assert_eq!(Dasha::disasm(&[0x1b, 0x34]), Err(Error::ExpectedSib));
    assert_eq!(Dasha::disasm(&[0x1b, 0x35]), Err(Error::ExpectedOffsetLong));
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x36]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Esi))),
            Val::Addr(Addr::Direct(Reg::Esi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x37]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Edi))),
            Val::Addr(Addr::Direct(Reg::Esi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x38]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Eax))),
            Val::Addr(Addr::Direct(Reg::Edi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x39]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Ecx))),
            Val::Addr(Addr::Direct(Reg::Edi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x3a]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Edx))),
            Val::Addr(Addr::Direct(Reg::Edi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x3b]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Ebx))),
            Val::Addr(Addr::Direct(Reg::Edi)),
        )])
    );
    assert_eq!(Dasha::disasm(&[0x1b, 0x3c]), Err(Error::ExpectedSib));
    assert_eq!(Dasha::disasm(&[0x1b, 0x3d]), Err(Error::ExpectedOffsetLong));
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x3e]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Esi))),
            Val::Addr(Addr::Direct(Reg::Edi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x3f]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Edi))),
            Val::Addr(Addr::Direct(Reg::Edi)),
        )])
    );
}

#[test]
fn test_indirect_mem_long_addressing() {
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x05, 0x00, 0x00, 0x00, 0x00]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Mem(Size::Long, Offset::I32(0)))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x05, 0xff, 0xff, 0xff, 0x7f]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Mem(
                Size::Long,
                Offset::I32(0x7fffffff)
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x05, 0x00, 0x00, 0x00, 0x80]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Mem(
                Size::Long,
                Offset::I32(-0x80000000)
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x05, 0xff, 0xff, 0xff, 0xff]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Mem(Size::Long, Offset::I32(-1)))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x0d, 0x00, 0x00, 0x00, 0x00]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Mem(Size::Long, Offset::I32(0)))),
            Val::Addr(Addr::Direct(Reg::Ecx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x0d, 0xff, 0xff, 0xff, 0x7f]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Mem(
                Size::Long,
                Offset::I32(0x7fffffff)
            ))),
            Val::Addr(Addr::Direct(Reg::Ecx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x0d, 0x00, 0x00, 0x00, 0x80]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Mem(
                Size::Long,
                Offset::I32(-0x80000000)
            ))),
            Val::Addr(Addr::Direct(Reg::Ecx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x0d, 0xff, 0xff, 0xff, 0xff]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Mem(Size::Long, Offset::I32(-1)))),
            Val::Addr(Addr::Direct(Reg::Ecx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x15, 0x00, 0x00, 0x00, 0x00]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Mem(Size::Long, Offset::I32(0)))),
            Val::Addr(Addr::Direct(Reg::Edx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x15, 0xff, 0xff, 0xff, 0x7f]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Mem(
                Size::Long,
                Offset::I32(0x7fffffff)
            ))),
            Val::Addr(Addr::Direct(Reg::Edx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x15, 0x00, 0x00, 0x00, 0x80]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Mem(
                Size::Long,
                Offset::I32(-0x80000000)
            ))),
            Val::Addr(Addr::Direct(Reg::Edx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x15, 0xff, 0xff, 0xff, 0xff]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Mem(Size::Long, Offset::I32(-1)))),
            Val::Addr(Addr::Direct(Reg::Edx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x1d, 0x00, 0x00, 0x00, 0x00]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Mem(Size::Long, Offset::I32(0)))),
            Val::Addr(Addr::Direct(Reg::Ebx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x1d, 0xff, 0xff, 0xff, 0x7f]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Mem(
                Size::Long,
                Offset::I32(0x7fffffff)
            ))),
            Val::Addr(Addr::Direct(Reg::Ebx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x1d, 0x00, 0x00, 0x00, 0x80]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Mem(
                Size::Long,
                Offset::I32(-0x80000000)
            ))),
            Val::Addr(Addr::Direct(Reg::Ebx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x1d, 0xff, 0xff, 0xff, 0xff]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Mem(Size::Long, Offset::I32(-1)))),
            Val::Addr(Addr::Direct(Reg::Ebx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x25, 0x00, 0x00, 0x00, 0x00]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Mem(Size::Long, Offset::I32(0)))),
            Val::Addr(Addr::Direct(Reg::Esp)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x25, 0xff, 0xff, 0xff, 0x7f]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Mem(
                Size::Long,
                Offset::I32(0x7fffffff)
            ))),
            Val::Addr(Addr::Direct(Reg::Esp)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x25, 0x00, 0x00, 0x00, 0x80]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Mem(
                Size::Long,
                Offset::I32(-0x80000000)
            ))),
            Val::Addr(Addr::Direct(Reg::Esp)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x25, 0xff, 0xff, 0xff, 0xff]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Mem(Size::Long, Offset::I32(-1)))),
            Val::Addr(Addr::Direct(Reg::Esp)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x2d, 0x00, 0x00, 0x00, 0x00]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Mem(Size::Long, Offset::I32(0)))),
            Val::Addr(Addr::Direct(Reg::Ebp)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x2d, 0xff, 0xff, 0xff, 0x7f]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Mem(
                Size::Long,
                Offset::I32(0x7fffffff)
            ))),
            Val::Addr(Addr::Direct(Reg::Ebp)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x2d, 0x00, 0x00, 0x00, 0x80]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Mem(
                Size::Long,
                Offset::I32(-0x80000000)
            ))),
            Val::Addr(Addr::Direct(Reg::Ebp)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x2d, 0xff, 0xff, 0xff, 0xff]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Mem(Size::Long, Offset::I32(-1)))),
            Val::Addr(Addr::Direct(Reg::Ebp)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x35, 0x00, 0x00, 0x00, 0x00]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Mem(Size::Long, Offset::I32(0)))),
            Val::Addr(Addr::Direct(Reg::Esi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x35, 0xff, 0xff, 0xff, 0x7f]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Mem(
                Size::Long,
                Offset::I32(0x7fffffff)
            ))),
            Val::Addr(Addr::Direct(Reg::Esi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x35, 0x00, 0x00, 0x00, 0x80]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Mem(
                Size::Long,
                Offset::I32(-0x80000000)
            ))),
            Val::Addr(Addr::Direct(Reg::Esi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x35, 0xff, 0xff, 0xff, 0xff]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Mem(Size::Long, Offset::I32(-1)))),
            Val::Addr(Addr::Direct(Reg::Esi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x3d, 0x00, 0x00, 0x00, 0x00]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Mem(Size::Long, Offset::I32(0)))),
            Val::Addr(Addr::Direct(Reg::Edi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x3d, 0xff, 0xff, 0xff, 0x7f]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Mem(
                Size::Long,
                Offset::I32(0x7fffffff)
            ))),
            Val::Addr(Addr::Direct(Reg::Edi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x3d, 0x00, 0x00, 0x00, 0x80]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Mem(
                Size::Long,
                Offset::I32(-0x80000000)
            ))),
            Val::Addr(Addr::Direct(Reg::Edi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x3d, 0xff, 0xff, 0xff, 0xff]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Mem(Size::Long, Offset::I32(-1)))),
            Val::Addr(Addr::Direct(Reg::Edi)),
        )])
    );
}

#[test]
fn test_indirect_sib_long_addressing() {
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x04, 0x00]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Eax,
                Reg::Eax,
                Scale::One
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x04, 0x01]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Ecx,
                Reg::Eax,
                Scale::One
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x04, 0x02]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Edx,
                Reg::Eax,
                Scale::One
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x04, 0x03]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Ebx,
                Reg::Eax,
                Scale::One
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x04, 0x04]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Esp,
                Reg::Eax,
                Scale::One
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x04, 0x05]),
        Err(Error::ExpectedOffsetLong)
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x04, 0x06]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Esi,
                Reg::Eax,
                Scale::One
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x04, 0x07]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Edi,
                Reg::Eax,
                Scale::One
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x04, 0x08]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Eax,
                Reg::Ecx,
                Scale::One
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x04, 0x09]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Ecx,
                Reg::Ecx,
                Scale::One
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x04, 0x0a]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Edx,
                Reg::Ecx,
                Scale::One
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x04, 0x0b]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Ebx,
                Reg::Ecx,
                Scale::One
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x04, 0x0c]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Esp,
                Reg::Ecx,
                Scale::One
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x04, 0x0d]),
        Err(Error::ExpectedOffsetLong)
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x04, 0x0e]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Esi,
                Reg::Ecx,
                Scale::One
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x04, 0x0f]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Edi,
                Reg::Ecx,
                Scale::One
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x04, 0x10]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Eax,
                Reg::Edx,
                Scale::One
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x04, 0x11]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Ecx,
                Reg::Edx,
                Scale::One
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x04, 0x12]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Edx,
                Reg::Edx,
                Scale::One
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x04, 0x13]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Ebx,
                Reg::Edx,
                Scale::One
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x04, 0x14]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Esp,
                Reg::Edx,
                Scale::One
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x04, 0x15]),
        Err(Error::ExpectedOffsetLong)
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x04, 0x16]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Esi,
                Reg::Edx,
                Scale::One
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x04, 0x17]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Edi,
                Reg::Edx,
                Scale::One
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x04, 0x18]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Eax,
                Reg::Ebx,
                Scale::One
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x04, 0x19]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Ecx,
                Reg::Ebx,
                Scale::One
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x04, 0x1a]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Edx,
                Reg::Ebx,
                Scale::One
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x04, 0x1b]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Ebx,
                Reg::Ebx,
                Scale::One
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x04, 0x1c]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Esp,
                Reg::Ebx,
                Scale::One
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x04, 0x1d]),
        Err(Error::ExpectedOffsetLong)
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x04, 0x1e]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Esi,
                Reg::Ebx,
                Scale::One
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x04, 0x1f]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Edi,
                Reg::Ebx,
                Scale::One
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x04, 0x20]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Eax,
                Reg::Eiz,
                Scale::One
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x04, 0x21]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Ecx,
                Reg::Eiz,
                Scale::One
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x04, 0x22]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Edx,
                Reg::Eiz,
                Scale::One
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x04, 0x23]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Ebx,
                Reg::Eiz,
                Scale::One
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x04, 0x24]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Esp))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x04, 0x25]),
        Err(Error::ExpectedOffsetLong)
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x04, 0x26]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Esi,
                Reg::Eiz,
                Scale::One
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x04, 0x27]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Edi,
                Reg::Eiz,
                Scale::One
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x04, 0x28]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Eax,
                Reg::Ebp,
                Scale::One
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x04, 0x29]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Ecx,
                Reg::Ebp,
                Scale::One
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x04, 0x2a]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Edx,
                Reg::Ebp,
                Scale::One
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x04, 0x2b]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Ebx,
                Reg::Ebp,
                Scale::One
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x04, 0x2c]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Esp,
                Reg::Ebp,
                Scale::One
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x04, 0x2d]),
        Err(Error::ExpectedOffsetLong)
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x04, 0x2e]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Esi,
                Reg::Ebp,
                Scale::One
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x04, 0x2f]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Edi,
                Reg::Ebp,
                Scale::One
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x04, 0x30]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Eax,
                Reg::Esi,
                Scale::One
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x04, 0x31]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Ecx,
                Reg::Esi,
                Scale::One
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x04, 0x32]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Edx,
                Reg::Esi,
                Scale::One
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x04, 0x33]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Ebx,
                Reg::Esi,
                Scale::One
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x04, 0x34]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Esp,
                Reg::Esi,
                Scale::One
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x04, 0x35]),
        Err(Error::ExpectedOffsetLong)
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x04, 0x36]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Esi,
                Reg::Esi,
                Scale::One
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x04, 0x37]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Edi,
                Reg::Esi,
                Scale::One
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x04, 0x38]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Eax,
                Reg::Edi,
                Scale::One
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x04, 0x39]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Ecx,
                Reg::Edi,
                Scale::One
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x04, 0x3a]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Edx,
                Reg::Edi,
                Scale::One
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x04, 0x3b]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Ebx,
                Reg::Edi,
                Scale::One
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x04, 0x3c]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Esp,
                Reg::Edi,
                Scale::One
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x04, 0x3d]),
        Err(Error::ExpectedOffsetLong)
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x04, 0x3e]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Esi,
                Reg::Edi,
                Scale::One
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x04, 0x3f]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Edi,
                Reg::Edi,
                Scale::One
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
}

#[test]
fn test_indirect_byte_offset_long_addressing() {
    assert_eq!(Dasha::disasm(&[0x1b, 0x40]), Err(Error::ExpectedOffsetByte));
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x40, 0x00]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I8(0),
                Reg::Eax
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x40, 0x7f]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I8(0x7f),
                Reg::Eax
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x40, 0x80]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I8(-0x80),
                Reg::Eax
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x40, 0xff]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I8(-1),
                Reg::Eax
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(Dasha::disasm(&[0x1b, 0x41]), Err(Error::ExpectedOffsetByte));
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x41, 0x00]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I8(0),
                Reg::Ecx
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x41, 0x7f]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I8(0x7f),
                Reg::Ecx
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x41, 0x80]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I8(-0x80),
                Reg::Ecx
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x41, 0xff]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I8(-1),
                Reg::Ecx
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(Dasha::disasm(&[0x1b, 0x42]), Err(Error::ExpectedOffsetByte));
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x42, 0x00]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I8(0),
                Reg::Edx
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x42, 0x7f]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I8(0x7f),
                Reg::Edx
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x42, 0x80]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I8(-0x80),
                Reg::Edx
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x42, 0xff]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I8(-1),
                Reg::Edx
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(Dasha::disasm(&[0x1b, 0x43]), Err(Error::ExpectedOffsetByte));
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x43, 0x00]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I8(0),
                Reg::Ebx
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x43, 0x7f]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I8(0x7f),
                Reg::Ebx
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x43, 0x80]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I8(-0x80),
                Reg::Ebx
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x43, 0xff]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I8(-1),
                Reg::Ebx
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(Dasha::disasm(&[0x1b, 0x44]), Err(Error::ExpectedSib));
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x44, 0x00]),
        Err(Error::ExpectedOffsetByte)
    );
    assert_eq!(Dasha::disasm(&[0x1b, 0x45]), Err(Error::ExpectedOffsetByte));
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x45, 0x00]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I8(0),
                Reg::Ebp
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x45, 0x7f]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I8(0x7f),
                Reg::Ebp
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x45, 0x80]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I8(-0x80),
                Reg::Ebp
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x45, 0xff]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I8(-1),
                Reg::Ebp
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(Dasha::disasm(&[0x1b, 0x46]), Err(Error::ExpectedOffsetByte));
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x46, 0x00]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I8(0),
                Reg::Esi
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x46, 0x7f]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I8(0x7f),
                Reg::Esi
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x46, 0x80]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I8(-0x80),
                Reg::Esi
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x46, 0xff]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I8(-1),
                Reg::Esi
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(Dasha::disasm(&[0x1b, 0x47]), Err(Error::ExpectedOffsetByte));
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x47, 0x00]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I8(0),
                Reg::Edi
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x47, 0x7f]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I8(0x7f),
                Reg::Edi
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x47, 0x80]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I8(-0x80),
                Reg::Edi
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x47, 0xff]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I8(-1),
                Reg::Edi
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
}

#[test]
fn test_indirect_byte_offset_sib_long_addressing() {
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x44, 0x00, 0x00]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::OffsetBaseIndexScale(
                Size::Long,
                Offset::I8(0),
                Reg::Eax,
                Reg::Eax,
                Scale::One
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x4c, 0x49, 0x7f]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::OffsetBaseIndexScale(
                Size::Long,
                Offset::I8(0x7f),
                Reg::Ecx,
                Reg::Ecx,
                Scale::Two
            ))),
            Val::Addr(Addr::Direct(Reg::Ecx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x54, 0x92, 0x80]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::OffsetBaseIndexScale(
                Size::Long,
                Offset::I8(-0x80),
                Reg::Edx,
                Reg::Edx,
                Scale::Four
            ))),
            Val::Addr(Addr::Direct(Reg::Edx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x5c, 0x9b, 0xff]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::OffsetBaseIndexScale(
                Size::Long,
                Offset::I8(-1),
                Reg::Ebx,
                Reg::Ebx,
                Scale::Four
            ))),
            Val::Addr(Addr::Direct(Reg::Ebx)),
        )])
    );
}

#[test]
fn test_indirect_long_offset_long_addressing() {
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x80, 0x00, 0x00, 0x80]),
        Err(Error::ExpectedOffsetLong)
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x80, 0x00, 0x00, 0x00, 0x00]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I32(0),
                Reg::Eax
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x80, 0xff, 0xff, 0xff, 0x7f]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I32(0x7fffffff),
                Reg::Eax
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x80, 0x00, 0x00, 0x00, 0x80]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I32(-0x80000000),
                Reg::Eax
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x80, 0xff, 0xff, 0xff, 0xff]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I32(-1),
                Reg::Eax
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(Dasha::disasm(&[0x1b, 0x81]), Err(Error::ExpectedOffsetLong));
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x81, 0x00, 0x00, 0x00, 0x00]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I32(0),
                Reg::Ecx
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x81, 0xff, 0xff, 0xff, 0x7f]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I32(0x7fffffff),
                Reg::Ecx
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x81, 0x00, 0x00, 0x00, 0x80]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I32(-0x80000000),
                Reg::Ecx
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x81, 0xff, 0xff, 0xff, 0xff]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I32(-1),
                Reg::Ecx
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(Dasha::disasm(&[0x1b, 0x82]), Err(Error::ExpectedOffsetLong));
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x82, 0x00, 0x00, 0x00, 0x00]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I32(0),
                Reg::Edx
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x82, 0xff, 0xff, 0xff, 0x7f]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I32(0x7fffffff),
                Reg::Edx
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x82, 0x00, 0x00, 0x00, 0x80]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I32(-0x80000000),
                Reg::Edx
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x82, 0xff, 0xff, 0xff, 0xff]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I32(-1),
                Reg::Edx
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(Dasha::disasm(&[0x1b, 0x83]), Err(Error::ExpectedOffsetLong));
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x83, 0x00, 0x00, 0x00, 0x00]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I32(0),
                Reg::Ebx
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x83, 0xff, 0xff, 0xff, 0x7f]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I32(0x7fffffff),
                Reg::Ebx
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x83, 0x00, 0x00, 0x00, 0x80]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I32(-0x80000000),
                Reg::Ebx
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x83, 0xff, 0xff, 0xff, 0xff]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I32(-1),
                Reg::Ebx
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(Dasha::disasm(&[0x1b, 0x84]), Err(Error::ExpectedSib));
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x84, 0x00, 0x00, 0x00, 0x00]),
        Err(Error::ExpectedOffsetLong)
    );
    assert_eq!(Dasha::disasm(&[0x1b, 0x85]), Err(Error::ExpectedOffsetLong));
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x85, 0x00, 0x00, 0x00, 0x00]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I32(0),
                Reg::Ebp
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x85, 0xff, 0xff, 0xff, 0x7f]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I32(0x7fffffff),
                Reg::Ebp
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x85, 0x00, 0x00, 0x00, 0x80]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I32(-0x80000000),
                Reg::Ebp
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x85, 0xff, 0xff, 0xff, 0xff]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I32(-1),
                Reg::Ebp
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(Dasha::disasm(&[0x1b, 0x86]), Err(Error::ExpectedOffsetLong));
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x86, 0x00, 0x00, 0x00, 0x00]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I32(0),
                Reg::Esi
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x86, 0xff, 0xff, 0xff, 0x7f]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I32(0x7fffffff),
                Reg::Esi
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x86, 0x00, 0x00, 0x00, 0x80]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I32(-0x80000000),
                Reg::Esi
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x86, 0xff, 0xff, 0xff, 0xff]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I32(-1),
                Reg::Esi
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(Dasha::disasm(&[0x1b, 0x87]), Err(Error::ExpectedOffsetLong));
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x87, 0x00, 0x00, 0x00, 0x00]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I32(0),
                Reg::Edi
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x87, 0xff, 0xff, 0xff, 0x7f]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I32(0x7fffffff),
                Reg::Edi
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x87, 0x00, 0x00, 0x00, 0x80]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I32(-0x80000000),
                Reg::Edi
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x87, 0xff, 0xff, 0xff, 0xff]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I32(-1),
                Reg::Edi
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
}

#[test]
fn test_indirect_long_offset_sib_long_addressing() {
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x84, 0x00, 0x00, 0x00, 0x00, 0x00]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::OffsetBaseIndexScale(
                Size::Long,
                Offset::I32(0),
                Reg::Eax,
                Reg::Eax,
                Scale::One
            ))),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x8c, 0x49, 0xff, 0xff, 0xff, 0x7f]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::OffsetBaseIndexScale(
                Size::Long,
                Offset::I32(0x7fffffff),
                Reg::Ecx,
                Reg::Ecx,
                Scale::Two
            ))),
            Val::Addr(Addr::Direct(Reg::Ecx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x94, 0x92, 0x00, 0x00, 0x00, 0x80]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::OffsetBaseIndexScale(
                Size::Long,
                Offset::I32(-0x80000000),
                Reg::Edx,
                Reg::Edx,
                Scale::Four
            ))),
            Val::Addr(Addr::Direct(Reg::Edx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x1b, 0x9c, 0x9b, 0xff, 0xff, 0xff, 0xff]),
        Ok(vec![Inst::Sbb(
            Val::Addr(Addr::Indirect(Indirect::OffsetBaseIndexScale(
                Size::Long,
                Offset::I32(-1),
                Reg::Ebx,
                Reg::Ebx,
                Scale::Four
            ))),
            Val::Addr(Addr::Direct(Reg::Ebx)),
        )])
    );
}
