use dasha::{Addr, Dasha, Error, Indirect, Inst, Offset, Reg, Scale, Size, Val};

#[test]
fn test_direct_long_addressing() {
    assert_eq!(
        Dasha::disasm(&[0x31, 0xc0]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0xc1]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Direct(Reg::Ecx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0xc2]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Direct(Reg::Edx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0xc3]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Direct(Reg::Ebx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0xc4]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Direct(Reg::Esp)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0xc5]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Direct(Reg::Ebp)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0xc6]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Direct(Reg::Esi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0xc7]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Direct(Reg::Edi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0xc8]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Ecx)),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0xc9]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Ecx)),
            Val::Addr(Addr::Direct(Reg::Ecx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0xca]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Ecx)),
            Val::Addr(Addr::Direct(Reg::Edx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0xcb]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Ecx)),
            Val::Addr(Addr::Direct(Reg::Ebx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0xcc]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Ecx)),
            Val::Addr(Addr::Direct(Reg::Esp)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0xcd]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Ecx)),
            Val::Addr(Addr::Direct(Reg::Ebp)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0xce]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Ecx)),
            Val::Addr(Addr::Direct(Reg::Esi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0xcf]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Ecx)),
            Val::Addr(Addr::Direct(Reg::Edi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0xd0]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Edx)),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0xd1]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Edx)),
            Val::Addr(Addr::Direct(Reg::Ecx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0xd2]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Edx)),
            Val::Addr(Addr::Direct(Reg::Edx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0xd3]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Edx)),
            Val::Addr(Addr::Direct(Reg::Ebx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0xd4]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Edx)),
            Val::Addr(Addr::Direct(Reg::Esp)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0xd5]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Edx)),
            Val::Addr(Addr::Direct(Reg::Ebp)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0xd6]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Edx)),
            Val::Addr(Addr::Direct(Reg::Esi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0xd7]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Edx)),
            Val::Addr(Addr::Direct(Reg::Edi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0xd8]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Ebx)),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0xd9]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Ebx)),
            Val::Addr(Addr::Direct(Reg::Ecx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0xda]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Ebx)),
            Val::Addr(Addr::Direct(Reg::Edx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0xdb]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Ebx)),
            Val::Addr(Addr::Direct(Reg::Ebx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0xdc]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Ebx)),
            Val::Addr(Addr::Direct(Reg::Esp)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0xdd]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Ebx)),
            Val::Addr(Addr::Direct(Reg::Ebp)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0xde]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Ebx)),
            Val::Addr(Addr::Direct(Reg::Esi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0xdf]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Ebx)),
            Val::Addr(Addr::Direct(Reg::Edi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0xe0]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Esp)),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0xe1]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Esp)),
            Val::Addr(Addr::Direct(Reg::Ecx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0xe2]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Esp)),
            Val::Addr(Addr::Direct(Reg::Edx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0xe3]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Esp)),
            Val::Addr(Addr::Direct(Reg::Ebx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0xe4]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Esp)),
            Val::Addr(Addr::Direct(Reg::Esp)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0xe5]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Esp)),
            Val::Addr(Addr::Direct(Reg::Ebp)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0xe6]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Esp)),
            Val::Addr(Addr::Direct(Reg::Esi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0xe7]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Esp)),
            Val::Addr(Addr::Direct(Reg::Edi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0xe8]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Ebp)),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0xe9]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Ebp)),
            Val::Addr(Addr::Direct(Reg::Ecx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0xea]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Ebp)),
            Val::Addr(Addr::Direct(Reg::Edx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0xeb]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Ebp)),
            Val::Addr(Addr::Direct(Reg::Ebx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0xec]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Ebp)),
            Val::Addr(Addr::Direct(Reg::Esp)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0xed]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Ebp)),
            Val::Addr(Addr::Direct(Reg::Ebp)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0xee]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Ebp)),
            Val::Addr(Addr::Direct(Reg::Esi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0xef]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Ebp)),
            Val::Addr(Addr::Direct(Reg::Edi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0xf0]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Esi)),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0xf1]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Esi)),
            Val::Addr(Addr::Direct(Reg::Ecx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0xf2]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Esi)),
            Val::Addr(Addr::Direct(Reg::Edx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0xf3]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Esi)),
            Val::Addr(Addr::Direct(Reg::Ebx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0xf4]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Esi)),
            Val::Addr(Addr::Direct(Reg::Esp)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0xf5]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Esi)),
            Val::Addr(Addr::Direct(Reg::Ebp)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0xf6]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Esi)),
            Val::Addr(Addr::Direct(Reg::Esi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0xf7]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Esi)),
            Val::Addr(Addr::Direct(Reg::Edi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0xf8]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Edi)),
            Val::Addr(Addr::Direct(Reg::Eax)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0xf9]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Edi)),
            Val::Addr(Addr::Direct(Reg::Ecx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0xfa]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Edi)),
            Val::Addr(Addr::Direct(Reg::Edx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0xfb]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Edi)),
            Val::Addr(Addr::Direct(Reg::Ebx)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0xfc]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Edi)),
            Val::Addr(Addr::Direct(Reg::Esp)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0xfd]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Edi)),
            Val::Addr(Addr::Direct(Reg::Ebp)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0xfe]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Edi)),
            Val::Addr(Addr::Direct(Reg::Esi)),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0xff]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Edi)),
            Val::Addr(Addr::Direct(Reg::Edi)),
        )])
    );
}

#[test]
fn test_indirect_base_long_addressing() {
    assert_eq!(
        Dasha::disasm(&[0x31, 0x00]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Eax))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x01]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Ecx))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x02]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Edx))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x03]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Ebx))),
        )])
    );
    assert_eq!(Dasha::disasm(&[0x31, 0x04]), Err(Error::ExpectedSib));
    assert_eq!(Dasha::disasm(&[0x31, 0x05]), Err(Error::ExpectedOffsetLong));
    assert_eq!(
        Dasha::disasm(&[0x31, 0x06]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Esi))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x07]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Edi))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x08]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Ecx)),
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Eax))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x09]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Ecx)),
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Ecx))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x0a]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Ecx)),
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Edx))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x0b]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Ecx)),
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Ebx))),
        )])
    );
    assert_eq!(Dasha::disasm(&[0x31, 0x0c]), Err(Error::ExpectedSib));
    assert_eq!(Dasha::disasm(&[0x31, 0x0d]), Err(Error::ExpectedOffsetLong));
    assert_eq!(
        Dasha::disasm(&[0x31, 0x0e]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Ecx)),
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Esi))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x0f]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Ecx)),
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Edi))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x10]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Edx)),
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Eax))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x11]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Edx)),
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Ecx))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x12]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Edx)),
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Edx))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x13]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Edx)),
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Ebx))),
        )])
    );
    assert_eq!(Dasha::disasm(&[0x31, 0x14]), Err(Error::ExpectedSib));
    assert_eq!(Dasha::disasm(&[0x31, 0x15]), Err(Error::ExpectedOffsetLong));
    assert_eq!(
        Dasha::disasm(&[0x31, 0x16]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Edx)),
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Esi))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x17]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Edx)),
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Edi))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x18]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Ebx)),
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Eax))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x19]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Ebx)),
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Ecx))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x1a]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Ebx)),
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Edx))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x1b]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Ebx)),
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Ebx))),
        )])
    );
    assert_eq!(Dasha::disasm(&[0x31, 0x1c]), Err(Error::ExpectedSib));
    assert_eq!(Dasha::disasm(&[0x31, 0x1d]), Err(Error::ExpectedOffsetLong));
    assert_eq!(
        Dasha::disasm(&[0x31, 0x1e]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Ebx)),
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Esi))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x1f]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Ebx)),
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Edi))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x20]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Esp)),
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Eax))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x21]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Esp)),
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Ecx))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x22]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Esp)),
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Edx))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x23]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Esp)),
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Ebx))),
        )])
    );
    assert_eq!(Dasha::disasm(&[0x31, 0x24]), Err(Error::ExpectedSib));
    assert_eq!(Dasha::disasm(&[0x31, 0x25]), Err(Error::ExpectedOffsetLong));
    assert_eq!(
        Dasha::disasm(&[0x31, 0x26]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Esp)),
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Esi))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x27]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Esp)),
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Edi))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x28]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Ebp)),
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Eax))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x29]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Ebp)),
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Ecx))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x2a]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Ebp)),
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Edx))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x2b]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Ebp)),
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Ebx))),
        )])
    );
    assert_eq!(Dasha::disasm(&[0x31, 0x2c]), Err(Error::ExpectedSib));
    assert_eq!(Dasha::disasm(&[0x31, 0x2d]), Err(Error::ExpectedOffsetLong));
    assert_eq!(
        Dasha::disasm(&[0x31, 0x2e]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Ebp)),
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Esi))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x2f]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Ebp)),
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Edi))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x30]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Esi)),
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Eax))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x31]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Esi)),
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Ecx))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x32]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Esi)),
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Edx))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x33]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Esi)),
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Ebx))),
        )])
    );
    assert_eq!(Dasha::disasm(&[0x31, 0x34]), Err(Error::ExpectedSib));
    assert_eq!(Dasha::disasm(&[0x31, 0x35]), Err(Error::ExpectedOffsetLong));
    assert_eq!(
        Dasha::disasm(&[0x31, 0x36]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Esi)),
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Esi))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x37]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Esi)),
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Edi))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x38]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Edi)),
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Eax))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x39]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Edi)),
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Ecx))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x3a]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Edi)),
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Edx))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x3b]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Edi)),
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Ebx))),
        )])
    );
    assert_eq!(Dasha::disasm(&[0x31, 0x3c]), Err(Error::ExpectedSib));
    assert_eq!(Dasha::disasm(&[0x31, 0x3d]), Err(Error::ExpectedOffsetLong));
    assert_eq!(
        Dasha::disasm(&[0x31, 0x3e]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Edi)),
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Esi))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x3f]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Edi)),
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Edi))),
        )])
    );
}

#[test]
fn test_indirect_mem_long_addressing() {
    assert_eq!(
        Dasha::disasm(&[0x31, 0x05, 0x00, 0x00, 0x00, 0x00]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::Mem(Size::Long, Offset::I32(0)))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x05, 0xff, 0xff, 0xff, 0x7f]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::Mem(
                Size::Long,
                Offset::I32(0x7fffffff)
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x05, 0x00, 0x00, 0x00, 0x80]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::Mem(
                Size::Long,
                Offset::I32(-0x80000000)
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x05, 0xff, 0xff, 0xff, 0xff]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::Mem(Size::Long, Offset::I32(-1)))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x0d, 0x00, 0x00, 0x00, 0x00]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Ecx)),
            Val::Addr(Addr::Indirect(Indirect::Mem(Size::Long, Offset::I32(0)))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x0d, 0xff, 0xff, 0xff, 0x7f]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Ecx)),
            Val::Addr(Addr::Indirect(Indirect::Mem(
                Size::Long,
                Offset::I32(0x7fffffff)
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x0d, 0x00, 0x00, 0x00, 0x80]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Ecx)),
            Val::Addr(Addr::Indirect(Indirect::Mem(
                Size::Long,
                Offset::I32(-0x80000000)
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x0d, 0xff, 0xff, 0xff, 0xff]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Ecx)),
            Val::Addr(Addr::Indirect(Indirect::Mem(Size::Long, Offset::I32(-1)))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x15, 0x00, 0x00, 0x00, 0x00]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Edx)),
            Val::Addr(Addr::Indirect(Indirect::Mem(Size::Long, Offset::I32(0)))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x15, 0xff, 0xff, 0xff, 0x7f]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Edx)),
            Val::Addr(Addr::Indirect(Indirect::Mem(
                Size::Long,
                Offset::I32(0x7fffffff)
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x15, 0x00, 0x00, 0x00, 0x80]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Edx)),
            Val::Addr(Addr::Indirect(Indirect::Mem(
                Size::Long,
                Offset::I32(-0x80000000)
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x15, 0xff, 0xff, 0xff, 0xff]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Edx)),
            Val::Addr(Addr::Indirect(Indirect::Mem(Size::Long, Offset::I32(-1)))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x1d, 0x00, 0x00, 0x00, 0x00]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Ebx)),
            Val::Addr(Addr::Indirect(Indirect::Mem(Size::Long, Offset::I32(0)))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x1d, 0xff, 0xff, 0xff, 0x7f]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Ebx)),
            Val::Addr(Addr::Indirect(Indirect::Mem(
                Size::Long,
                Offset::I32(0x7fffffff)
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x1d, 0x00, 0x00, 0x00, 0x80]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Ebx)),
            Val::Addr(Addr::Indirect(Indirect::Mem(
                Size::Long,
                Offset::I32(-0x80000000)
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x1d, 0xff, 0xff, 0xff, 0xff]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Ebx)),
            Val::Addr(Addr::Indirect(Indirect::Mem(Size::Long, Offset::I32(-1)))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x25, 0x00, 0x00, 0x00, 0x00]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Esp)),
            Val::Addr(Addr::Indirect(Indirect::Mem(Size::Long, Offset::I32(0)))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x25, 0xff, 0xff, 0xff, 0x7f]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Esp)),
            Val::Addr(Addr::Indirect(Indirect::Mem(
                Size::Long,
                Offset::I32(0x7fffffff)
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x25, 0x00, 0x00, 0x00, 0x80]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Esp)),
            Val::Addr(Addr::Indirect(Indirect::Mem(
                Size::Long,
                Offset::I32(-0x80000000)
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x25, 0xff, 0xff, 0xff, 0xff]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Esp)),
            Val::Addr(Addr::Indirect(Indirect::Mem(Size::Long, Offset::I32(-1)))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x2d, 0x00, 0x00, 0x00, 0x00]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Ebp)),
            Val::Addr(Addr::Indirect(Indirect::Mem(Size::Long, Offset::I32(0)))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x2d, 0xff, 0xff, 0xff, 0x7f]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Ebp)),
            Val::Addr(Addr::Indirect(Indirect::Mem(
                Size::Long,
                Offset::I32(0x7fffffff)
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x2d, 0x00, 0x00, 0x00, 0x80]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Ebp)),
            Val::Addr(Addr::Indirect(Indirect::Mem(
                Size::Long,
                Offset::I32(-0x80000000)
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x2d, 0xff, 0xff, 0xff, 0xff]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Ebp)),
            Val::Addr(Addr::Indirect(Indirect::Mem(Size::Long, Offset::I32(-1)))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x35, 0x00, 0x00, 0x00, 0x00]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Esi)),
            Val::Addr(Addr::Indirect(Indirect::Mem(Size::Long, Offset::I32(0)))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x35, 0xff, 0xff, 0xff, 0x7f]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Esi)),
            Val::Addr(Addr::Indirect(Indirect::Mem(
                Size::Long,
                Offset::I32(0x7fffffff)
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x35, 0x00, 0x00, 0x00, 0x80]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Esi)),
            Val::Addr(Addr::Indirect(Indirect::Mem(
                Size::Long,
                Offset::I32(-0x80000000)
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x35, 0xff, 0xff, 0xff, 0xff]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Esi)),
            Val::Addr(Addr::Indirect(Indirect::Mem(Size::Long, Offset::I32(-1)))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x3d, 0x00, 0x00, 0x00, 0x00]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Edi)),
            Val::Addr(Addr::Indirect(Indirect::Mem(Size::Long, Offset::I32(0)))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x3d, 0xff, 0xff, 0xff, 0x7f]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Edi)),
            Val::Addr(Addr::Indirect(Indirect::Mem(
                Size::Long,
                Offset::I32(0x7fffffff)
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x3d, 0x00, 0x00, 0x00, 0x80]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Edi)),
            Val::Addr(Addr::Indirect(Indirect::Mem(
                Size::Long,
                Offset::I32(-0x80000000)
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x3d, 0xff, 0xff, 0xff, 0xff]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Edi)),
            Val::Addr(Addr::Indirect(Indirect::Mem(Size::Long, Offset::I32(-1)))),
        )])
    );
}

#[test]
fn test_indirect_sib_long_addressing() {
    assert_eq!(
        Dasha::disasm(&[0x31, 0x04, 0x00]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Eax,
                Reg::Eax,
                Scale::One
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x04, 0x01]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Ecx,
                Reg::Eax,
                Scale::One
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x04, 0x02]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Edx,
                Reg::Eax,
                Scale::One
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x04, 0x03]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Ebx,
                Reg::Eax,
                Scale::One
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x04, 0x04]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Esp,
                Reg::Eax,
                Scale::One
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x04, 0x05]),
        Err(Error::ExpectedOffsetLong)
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x04, 0x06]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Esi,
                Reg::Eax,
                Scale::One
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x04, 0x07]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Edi,
                Reg::Eax,
                Scale::One
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x04, 0x08]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Eax,
                Reg::Ecx,
                Scale::One
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x04, 0x09]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Ecx,
                Reg::Ecx,
                Scale::One
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x04, 0x0a]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Edx,
                Reg::Ecx,
                Scale::One
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x04, 0x0b]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Ebx,
                Reg::Ecx,
                Scale::One
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x04, 0x0c]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Esp,
                Reg::Ecx,
                Scale::One
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x04, 0x0d]),
        Err(Error::ExpectedOffsetLong)
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x04, 0x0e]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Esi,
                Reg::Ecx,
                Scale::One
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x04, 0x0f]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Edi,
                Reg::Ecx,
                Scale::One
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x04, 0x10]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Eax,
                Reg::Edx,
                Scale::One
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x04, 0x11]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Ecx,
                Reg::Edx,
                Scale::One
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x04, 0x12]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Edx,
                Reg::Edx,
                Scale::One
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x04, 0x13]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Ebx,
                Reg::Edx,
                Scale::One
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x04, 0x14]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Esp,
                Reg::Edx,
                Scale::One
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x04, 0x15]),
        Err(Error::ExpectedOffsetLong)
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x04, 0x16]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Esi,
                Reg::Edx,
                Scale::One
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x04, 0x17]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Edi,
                Reg::Edx,
                Scale::One
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x04, 0x18]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Eax,
                Reg::Ebx,
                Scale::One
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x04, 0x19]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Ecx,
                Reg::Ebx,
                Scale::One
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x04, 0x1a]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Edx,
                Reg::Ebx,
                Scale::One
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x04, 0x1b]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Ebx,
                Reg::Ebx,
                Scale::One
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x04, 0x1c]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Esp,
                Reg::Ebx,
                Scale::One
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x04, 0x1d]),
        Err(Error::ExpectedOffsetLong)
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x04, 0x1e]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Esi,
                Reg::Ebx,
                Scale::One
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x04, 0x1f]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Edi,
                Reg::Ebx,
                Scale::One
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x04, 0x20]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Eax,
                Reg::Eiz,
                Scale::One
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x04, 0x21]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Ecx,
                Reg::Eiz,
                Scale::One
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x04, 0x22]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Edx,
                Reg::Eiz,
                Scale::One
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x04, 0x23]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Ebx,
                Reg::Eiz,
                Scale::One
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x04, 0x24]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::Base(Size::Long, Reg::Esp))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x04, 0x25]),
        Err(Error::ExpectedOffsetLong)
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x04, 0x26]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Esi,
                Reg::Eiz,
                Scale::One
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x04, 0x27]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Edi,
                Reg::Eiz,
                Scale::One
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x04, 0x28]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Eax,
                Reg::Ebp,
                Scale::One
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x04, 0x29]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Ecx,
                Reg::Ebp,
                Scale::One
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x04, 0x2a]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Edx,
                Reg::Ebp,
                Scale::One
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x04, 0x2b]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Ebx,
                Reg::Ebp,
                Scale::One
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x04, 0x2c]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Esp,
                Reg::Ebp,
                Scale::One
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x04, 0x2d]),
        Err(Error::ExpectedOffsetLong)
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x04, 0x2e]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Esi,
                Reg::Ebp,
                Scale::One
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x04, 0x2f]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Edi,
                Reg::Ebp,
                Scale::One
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x04, 0x30]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Eax,
                Reg::Esi,
                Scale::One
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x04, 0x31]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Ecx,
                Reg::Esi,
                Scale::One
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x04, 0x32]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Edx,
                Reg::Esi,
                Scale::One
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x04, 0x33]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Ebx,
                Reg::Esi,
                Scale::One
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x04, 0x34]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Esp,
                Reg::Esi,
                Scale::One
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x04, 0x35]),
        Err(Error::ExpectedOffsetLong)
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x04, 0x36]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Esi,
                Reg::Esi,
                Scale::One
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x04, 0x37]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Edi,
                Reg::Esi,
                Scale::One
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x04, 0x38]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Eax,
                Reg::Edi,
                Scale::One
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x04, 0x39]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Ecx,
                Reg::Edi,
                Scale::One
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x04, 0x3a]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Edx,
                Reg::Edi,
                Scale::One
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x04, 0x3b]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Ebx,
                Reg::Edi,
                Scale::One
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x04, 0x3c]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Esp,
                Reg::Edi,
                Scale::One
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x04, 0x3d]),
        Err(Error::ExpectedOffsetLong)
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x04, 0x3e]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Esi,
                Reg::Edi,
                Scale::One
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x04, 0x3f]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::BaseIndexScale(
                Size::Long,
                Reg::Edi,
                Reg::Edi,
                Scale::One
            ))),
        )])
    );
}

#[test]
fn test_indirect_byte_offset_long_addressing() {
    assert_eq!(Dasha::disasm(&[0x31, 0x40]), Err(Error::ExpectedOffsetByte));
    assert_eq!(
        Dasha::disasm(&[0x31, 0x40, 0x00]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I8(0),
                Reg::Eax
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x40, 0x7f]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I8(0x7f),
                Reg::Eax
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x40, 0x80]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I8(-0x80),
                Reg::Eax
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x40, 0xff]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I8(-1),
                Reg::Eax
            ))),
        )])
    );
    assert_eq!(Dasha::disasm(&[0x31, 0x41]), Err(Error::ExpectedOffsetByte));
    assert_eq!(
        Dasha::disasm(&[0x31, 0x41, 0x00]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I8(0),
                Reg::Ecx
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x41, 0x7f]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I8(0x7f),
                Reg::Ecx
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x41, 0x80]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I8(-0x80),
                Reg::Ecx
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x41, 0xff]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I8(-1),
                Reg::Ecx
            ))),
        )])
    );
    assert_eq!(Dasha::disasm(&[0x31, 0x42]), Err(Error::ExpectedOffsetByte));
    assert_eq!(
        Dasha::disasm(&[0x31, 0x42, 0x00]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I8(0),
                Reg::Edx
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x42, 0x7f]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I8(0x7f),
                Reg::Edx
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x42, 0x80]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I8(-0x80),
                Reg::Edx
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x42, 0xff]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I8(-1),
                Reg::Edx
            ))),
        )])
    );
    assert_eq!(Dasha::disasm(&[0x31, 0x43]), Err(Error::ExpectedOffsetByte));
    assert_eq!(
        Dasha::disasm(&[0x31, 0x43, 0x00]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I8(0),
                Reg::Ebx
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x43, 0x7f]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I8(0x7f),
                Reg::Ebx
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x43, 0x80]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I8(-0x80),
                Reg::Ebx
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x43, 0xff]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I8(-1),
                Reg::Ebx
            ))),
        )])
    );
    assert_eq!(Dasha::disasm(&[0x31, 0x44]), Err(Error::ExpectedSib));
    assert_eq!(
        Dasha::disasm(&[0x31, 0x44, 0x00]),
        Err(Error::ExpectedOffsetByte)
    );
    assert_eq!(Dasha::disasm(&[0x31, 0x45]), Err(Error::ExpectedOffsetByte));
    assert_eq!(
        Dasha::disasm(&[0x31, 0x45, 0x00]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I8(0),
                Reg::Ebp
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x45, 0x7f]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I8(0x7f),
                Reg::Ebp
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x45, 0x80]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I8(-0x80),
                Reg::Ebp
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x45, 0xff]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I8(-1),
                Reg::Ebp
            ))),
        )])
    );
    assert_eq!(Dasha::disasm(&[0x31, 0x46]), Err(Error::ExpectedOffsetByte));
    assert_eq!(
        Dasha::disasm(&[0x31, 0x46, 0x00]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I8(0),
                Reg::Esi
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x46, 0x7f]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I8(0x7f),
                Reg::Esi
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x46, 0x80]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I8(-0x80),
                Reg::Esi
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x46, 0xff]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I8(-1),
                Reg::Esi
            ))),
        )])
    );
    assert_eq!(Dasha::disasm(&[0x31, 0x47]), Err(Error::ExpectedOffsetByte));
    assert_eq!(
        Dasha::disasm(&[0x31, 0x47, 0x00]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I8(0),
                Reg::Edi
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x47, 0x7f]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I8(0x7f),
                Reg::Edi
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x47, 0x80]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I8(-0x80),
                Reg::Edi
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x47, 0xff]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I8(-1),
                Reg::Edi
            ))),
        )])
    );
}

#[test]
fn test_indirect_byte_offset_sib_long_addressing() {
    assert_eq!(
        Dasha::disasm(&[0x31, 0x44, 0x00, 0x00]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::OffsetBaseIndexScale(
                Size::Long,
                Offset::I8(0),
                Reg::Eax,
                Reg::Eax,
                Scale::One
            )))
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x4c, 0x49, 0x7f]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Ecx)),
            Val::Addr(Addr::Indirect(Indirect::OffsetBaseIndexScale(
                Size::Long,
                Offset::I8(0x7f),
                Reg::Ecx,
                Reg::Ecx,
                Scale::Two
            )))
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x54, 0x92, 0x80]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Edx)),
            Val::Addr(Addr::Indirect(Indirect::OffsetBaseIndexScale(
                Size::Long,
                Offset::I8(-0x80),
                Reg::Edx,
                Reg::Edx,
                Scale::Four
            )))
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x5c, 0x9b, 0xff]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Ebx)),
            Val::Addr(Addr::Indirect(Indirect::OffsetBaseIndexScale(
                Size::Long,
                Offset::I8(-1),
                Reg::Ebx,
                Reg::Ebx,
                Scale::Four
            )))
        )])
    );
}

#[test]
fn test_indirect_long_offset_long_addressing() {
    assert_eq!(
        Dasha::disasm(&[0x31, 0x80, 0x00, 0x00, 0x80]),
        Err(Error::ExpectedOffsetLong)
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x80, 0x00, 0x00, 0x00, 0x00]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I32(0),
                Reg::Eax
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x80, 0xff, 0xff, 0xff, 0x7f]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I32(0x7fffffff),
                Reg::Eax
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x80, 0x00, 0x00, 0x00, 0x80]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I32(-0x80000000),
                Reg::Eax
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x80, 0xff, 0xff, 0xff, 0xff]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I32(-1),
                Reg::Eax
            ))),
        )])
    );
    assert_eq!(Dasha::disasm(&[0x31, 0x81]), Err(Error::ExpectedOffsetLong));
    assert_eq!(
        Dasha::disasm(&[0x31, 0x81, 0x00, 0x00, 0x00, 0x00]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I32(0),
                Reg::Ecx
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x81, 0xff, 0xff, 0xff, 0x7f]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I32(0x7fffffff),
                Reg::Ecx
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x81, 0x00, 0x00, 0x00, 0x80]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I32(-0x80000000),
                Reg::Ecx
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x81, 0xff, 0xff, 0xff, 0xff]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I32(-1),
                Reg::Ecx
            ))),
        )])
    );
    assert_eq!(Dasha::disasm(&[0x31, 0x82]), Err(Error::ExpectedOffsetLong));
    assert_eq!(
        Dasha::disasm(&[0x31, 0x82, 0x00, 0x00, 0x00, 0x00]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I32(0),
                Reg::Edx
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x82, 0xff, 0xff, 0xff, 0x7f]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I32(0x7fffffff),
                Reg::Edx
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x82, 0x00, 0x00, 0x00, 0x80]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I32(-0x80000000),
                Reg::Edx
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x82, 0xff, 0xff, 0xff, 0xff]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I32(-1),
                Reg::Edx
            ))),
        )])
    );
    assert_eq!(Dasha::disasm(&[0x31, 0x83]), Err(Error::ExpectedOffsetLong));
    assert_eq!(
        Dasha::disasm(&[0x31, 0x83, 0x00, 0x00, 0x00, 0x00]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I32(0),
                Reg::Ebx
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x83, 0xff, 0xff, 0xff, 0x7f]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I32(0x7fffffff),
                Reg::Ebx
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x83, 0x00, 0x00, 0x00, 0x80]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I32(-0x80000000),
                Reg::Ebx
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x83, 0xff, 0xff, 0xff, 0xff]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I32(-1),
                Reg::Ebx
            ))),
        )])
    );
    assert_eq!(Dasha::disasm(&[0x31, 0x84]), Err(Error::ExpectedSib));
    assert_eq!(
        Dasha::disasm(&[0x31, 0x84, 0x00, 0x00, 0x00, 0x00]),
        Err(Error::ExpectedOffsetLong)
    );
    assert_eq!(Dasha::disasm(&[0x31, 0x85]), Err(Error::ExpectedOffsetLong));
    assert_eq!(
        Dasha::disasm(&[0x31, 0x85, 0x00, 0x00, 0x00, 0x00]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I32(0),
                Reg::Ebp
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x85, 0xff, 0xff, 0xff, 0x7f]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I32(0x7fffffff),
                Reg::Ebp
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x85, 0x00, 0x00, 0x00, 0x80]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I32(-0x80000000),
                Reg::Ebp
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x85, 0xff, 0xff, 0xff, 0xff]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I32(-1),
                Reg::Ebp
            ))),
        )])
    );
    assert_eq!(Dasha::disasm(&[0x31, 0x86]), Err(Error::ExpectedOffsetLong));
    assert_eq!(
        Dasha::disasm(&[0x31, 0x86, 0x00, 0x00, 0x00, 0x00]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I32(0),
                Reg::Esi
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x86, 0xff, 0xff, 0xff, 0x7f]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I32(0x7fffffff),
                Reg::Esi
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x86, 0x00, 0x00, 0x00, 0x80]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I32(-0x80000000),
                Reg::Esi
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x86, 0xff, 0xff, 0xff, 0xff]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I32(-1),
                Reg::Esi
            ))),
        )])
    );
    assert_eq!(Dasha::disasm(&[0x31, 0x87]), Err(Error::ExpectedOffsetLong));
    assert_eq!(
        Dasha::disasm(&[0x31, 0x87, 0x00, 0x00, 0x00, 0x00]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I32(0),
                Reg::Edi
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x87, 0xff, 0xff, 0xff, 0x7f]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I32(0x7fffffff),
                Reg::Edi
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x87, 0x00, 0x00, 0x00, 0x80]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I32(-0x80000000),
                Reg::Edi
            ))),
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x87, 0xff, 0xff, 0xff, 0xff]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::OffsetBase(
                Size::Long,
                Offset::I32(-1),
                Reg::Edi
            ))),
        )])
    );
}

#[test]
fn test_indirect_long_offset_sib_long_addressing() {
    assert_eq!(
        Dasha::disasm(&[0x31, 0x84, 0x00, 0x00, 0x00, 0x00, 0x00]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Eax)),
            Val::Addr(Addr::Indirect(Indirect::OffsetBaseIndexScale(
                Size::Long,
                Offset::I32(0),
                Reg::Eax,
                Reg::Eax,
                Scale::One
            )))
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x8c, 0x49, 0xff, 0xff, 0xff, 0x7f]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Ecx)),
            Val::Addr(Addr::Indirect(Indirect::OffsetBaseIndexScale(
                Size::Long,
                Offset::I32(0x7fffffff),
                Reg::Ecx,
                Reg::Ecx,
                Scale::Two
            )))
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x94, 0x92, 0x00, 0x00, 0x00, 0x80]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Edx)),
            Val::Addr(Addr::Indirect(Indirect::OffsetBaseIndexScale(
                Size::Long,
                Offset::I32(-0x80000000),
                Reg::Edx,
                Reg::Edx,
                Scale::Four
            )))
        )])
    );
    assert_eq!(
        Dasha::disasm(&[0x31, 0x9c, 0x9b, 0xff, 0xff, 0xff, 0xff]),
        Ok(vec![Inst::Xor(
            Val::Addr(Addr::Direct(Reg::Ebx)),
            Val::Addr(Addr::Indirect(Indirect::OffsetBaseIndexScale(
                Size::Long,
                Offset::I32(-1),
                Reg::Ebx,
                Reg::Ebx,
                Scale::Four
            )))
        )])
    );
}
