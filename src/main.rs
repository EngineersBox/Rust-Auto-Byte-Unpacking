#[derive(Debug,Default)]
struct Other {
    pub f: u8,
}

byte_layout!(
    Other
    value [f, u8]
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
    pub l: Other,
    pub m: Vec<u8>,
}

byte_layout!(
    TestStruct
    value [a, u8]
    value [b, u16, Big]
    bytes_vec [c, b]
    value [d, u8]
    composite_vec [e, d, Other]
    value [g, u32, Big]
    primitive_vec [h, g, u16, Big]
    bytes_vec_lit [i, 2]
    primitive_vec_lit [j, 2, u16, Big]
    composite_vec_lit [k, 2, Other]
    composite [l, Other]
    bytes_vec_null_term [m]
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
        0x43, 0x21,
        0x55,
        0x01, 0x02, 0x03, 0x04, 0x05, 0x00
    ];
    let mut test_struct: TestStruct = Default::default();
    println!("New: {:#04X?}", test_struct);
    test_struct.parse_bytes::<&'_ [u8], nom::error::Error<_>>(test.as_slice());
    println!("Parsed: {:#04X?}", test_struct);
}