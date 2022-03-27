# Rust-Auto-Byte-Unpacking
A rust macro to define byte unpacking for structs

## Example

```rust
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
    let mut test_struct: TestStruct = Default::default();
    println!("New: {:#04X?}", test_struct);
    /* Prints:
     * New: TestStruct {
     *     a: 0x00,
     *     b: 0x00,
     *     c: [],
     *     d: 0x00,
     *     e: [],
     * }
     */
    test_struct.parse_bytes::<&'_ [u8], nom::error::Error<_>>(bytes.as_slice());
    println!("Parsed: {:#04X?}", test_struct);
    /* Prints:
     * Parsed: TestStruct {
     *     a: 0x2F,
     *     b: 0x02,
     *     c: [
     *         0xBE,
     *         0xEF,
     *     ],
     *     d: 0x02,
     *     e: [
     *         Other {
     *             f: 0xDE,
     *         },
     *         Other {
     *             f: 0xAD,
     *         },
     *     ],
     * }
     */
}
```
