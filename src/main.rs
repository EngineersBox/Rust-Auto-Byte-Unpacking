#[derive(Debug,Default)]
struct Other {
    pub f: u8,
}

impl Other {
    pub fn new() -> Other {
        return Other {
            f: 0,
        };
    }
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
}

impl TestStruct {
    pub fn new() -> TestStruct {
        return TestStruct {
            a: 0,
            b: 0,
            c: Vec::new(),
            d: 0,
            e: Vec::new(),
        };
    }
}

byte_layout!(
    TestStruct
    value [a, {nom::number::complete::be_u8::<I,E>}]
    value [b, {nom::number::complete::be_u16::<I,E>}]
    pure_vec [c, b]
    value [d, {nom::number::complete::be_u8::<I,E>}]
    typed_vec [e, d, Other]
);

fn main() {
    let bytes: Vec<u8> = vec![0x2F, 0x00, 0x02, 0xBE, 0xEF, 0x02, 0xDE, 0xAD];
    let mut test_struct: TestStruct = TestStruct::new();
    println!("New: {:#04X?}", test_struct);
    test_struct.parse_bytes::<&'_ [u8], nom::error::Error<_>>(bytes.as_slice());
    println!("Parsed: {:#04X?}", test_struct);
}