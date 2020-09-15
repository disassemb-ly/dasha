use std::io::{self, Read, Write};

use byteorder::{LittleEndian, WriteBytesExt};

fn unescape(data: &[u8]) -> Result<Vec<u8>, ()> {
    let mut iter = data.iter().copied();
    let mut data = vec![];

    loop {
        data.push(match iter.next() {
            Some(b'\\') => match iter.next() {
                Some(b'0') => 0,
                Some(b'x') => match (iter.next(), iter.next()) {
                    (Some(l), Some(r)) => {
                        u8::from_str_radix(&format!("{}{}", l as char, r as char), 16)
                            .or(Err(()))?
                    }
                    _ => unimplemented!(),
                },
                None => break Err(()),
                _ => unimplemented!(),
            },
            Some(byte) => byte,
            None => break Ok(data),
        });
    }
}

fn main() -> io::Result<()> {
    let data = {
        let mut data = vec![];
        io::stdin().read_to_end(&mut data)?;
        unescape(&data).unwrap()
    };

    let padded = {
        let mut padded = data.clone();
        padded.resize(
            if data.len() % 4 == 0 {
                data.len()
            } else {
                data.len() + 4 - data.len() % 4
            },
            0,
        );
        padded
    };

    let mut stdout = io::stdout();
    stdout.write_u32::<LittleEndian>(0xfeedface)?;
    stdout.write_i32::<LittleEndian>(7)?; // CPU_TYPE_I386
    stdout.write_i32::<LittleEndian>(3)?; // CPU_SUBTYPE_386
    stdout.write_u32::<LittleEndian>(1)?;
    stdout.write_u32::<LittleEndian>(2)?;
    stdout.write_u32::<LittleEndian>(0x94)?;
    stdout.write_u32::<LittleEndian>(0)?;

    stdout.write_u32::<LittleEndian>(1)?;
    stdout.write_u32::<LittleEndian>(0x7c)?;

    stdout.write_u32::<LittleEndian>(0)?;
    stdout.write_u32::<LittleEndian>(0)?;
    stdout.write_u32::<LittleEndian>(0)?;
    stdout.write_u32::<LittleEndian>(0)?;
    stdout.write_u32::<LittleEndian>(0)?;
    stdout.write_u32::<LittleEndian>(data.len() as _)?;
    stdout.write_u32::<LittleEndian>(0xb0)?;
    stdout.write_u32::<LittleEndian>(data.len() as _)?;
    stdout.write_u32::<LittleEndian>(7)?;
    stdout.write_u32::<LittleEndian>(7)?;
    stdout.write_u32::<LittleEndian>(1)?;
    stdout.write_u32::<LittleEndian>(0)?;
    stdout.write_all(b"__text\0\0\0\0\0\0\0\0\0\0")?; // sectname
    stdout.write_all(b"__TEXT\0\0\0\0\0\0\0\0\0\0")?; // segname
    stdout.write_u32::<LittleEndian>(0)?; // addr
    stdout.write_u32::<LittleEndian>(data.len() as _)?; // size
    stdout.write_u32::<LittleEndian>(0xb0)?; // offset
    stdout.write_u32::<LittleEndian>(0)?; // align
    stdout.write_u32::<LittleEndian>(0)?; // reloff
    stdout.write_u32::<LittleEndian>(0)?; // nreloc
    stdout.write_u32::<LittleEndian>(0x80000400)?; // flags
    stdout.write_u32::<LittleEndian>(0)?; // reserved
    stdout.write_u32::<LittleEndian>(0)?; // reserved
    stdout.write_u32::<LittleEndian>(0)?; // reserved
    stdout.write_u32::<LittleEndian>(0x18)?;
    stdout.write_u32::<LittleEndian>(1)?;
    stdout.write_u32::<LittleEndian>(0x000a0f00)?;
    stdout.write_u32::<LittleEndian>(0)?;
    stdout.write_u32::<LittleEndian>(0)?;

    stdout.write_all(&padded)?;

    Ok(())
}
