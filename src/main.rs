#[derive(Debug,Default)]
struct Other {
    pub f: u8,
}

byte_layout!(
    Other
    value [f, {nom::number::complete::be_u8::<I,E>}]
);

#[derive(Debug,Default)]
struct TestStruct {
    pub a: u8,
    pub b: u16,
    pub c: Vec<u8>,
    pub d: u8,
    pub e: Vec<Other>,
    pub g: u32,
    pub h: Vec<u16>,
    pub i: Vec<u8>,
    pub j: Vec<u16>,
    pub k: Vec<Other>,
}

byte_layout!(
    TestStruct
    value [a, {nom::number::complete::be_u8::<I,E>}]
    value [b, {nom::number::complete::be_u16::<I,E>}]
    bytes_vec [c, b]
    value [d, {nom::number::complete::be_u8::<I,E>}]
    composite_vec [e, d, Other]
    value [g, {nom::number::complete::be_u32::<I,E>}]
    primitive_vec [h, g, {nom::number::complete::be_u16::<I,E>}]
    bytes_vec_lit [i, 2]
    primitive_vec_lit [j, 2, {nom::number::complete::be_u16::<I,E>}]
    composite_vec_lit [k, 2, Other]
);

fn main() {
    let test: Vec<u8> = vec![
        0x2F,
        0x00, 0x02,
        0xBE, 0xEF,
        0x02,
        0xDE, 0xAD,
        0x00, 0x00, 0x00, 0x02,
        0x12, 0x34,
        0x56, 0x78,
        0x01,
        0x10,
        0xDE, 0xAD,
        0xBE, 0xEF,
        0x43, 0x21
    ];
    let mut test_struct: TestStruct = Default::default();
    println!("New: {:#04X?}", test_struct);
    test_struct.parse_bytes::<&'_ [u8], nom::error::Error<_>>(test.as_slice());
    println!("Parsed: {:#04X?}", test_struct);
}