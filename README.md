# Rust-Auto-Byte-Unpacking
A rust macro to define byte unpacking for structs

## Usage

### Macro syntax

```rust
byte_layout!{
    <struct_name>
    (<layout_type> [<args>])+
}
```

### Layout Types

_*Note*_: The use of `[...]?` here indicates an optional value.

| Name                        	| Syntax                                                                            	| Description                                                                                                                                                                                                                                                                                     	|
|-----------------------------	|-----------------------------------------------------------------------------------	|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------	|
| Value                       	| `value [<target field>, <number type>[, <endianness>]?]`                          	| Read a single value as an number type into a target field with a given endianness.                                                                                                                                                                                                              	|
| Bytes Array                 	| `bytes_vec [<target field>, <byte count field>]`                                  	| Read n bytes into a `Vec<u8>`. The number of bytes read is specified in the byte count field.                                                                                                                                                                                                   	|
| Literal Bytes Array         	| `bytes_vec_lit [<target field> <int count>]`                                      	| Read n bytes into a `Vec<u8>`. The number of bytes read is specified by the literal integer provided.                                                                                                                                                                                           	|
| Null Terminated Bytes Array 	| `bytes_vec_null_term [<target field>]`                                            	| Read n bytes until a null byte (`0x00`) is encounter and store in target field as `Vec<u8>`                                                                                                                                                                                                     	|
| Primitive Array             	| `primitive_vec [<target field> <num count field> <number type>[, <endianness>]?]` 	| Read n primitive numbers with a given endianness into a field as a `Vec<_>`.<br>The amount of numbers read is specified in the in the number count field.                                                                                                                                       	|
| Literal Primitive Array     	| `primitive_vec_lit [<target field> <int count> <number type>[, <endianness>]?]`   	| Read n primitive numbers with a given endianness into a field as a `Vec<_>`.<br>The amount of numbers read is specified by the literal integer provided.                                                                                                                                        	|
| Composite                   	| `composite [<target field> <struct name>]`                                        	| Parse bytes required for a given struct into the target field. Note that the struct must have<br>`byte_layout!{}` declared on it and `#[derive(Default)]` as `parse_bytes::<_,_>(tail)` is called to a default instance of it.                                                                  	|
| Composite Array             	| `composite_vec [<target field> <num count field> <struct name>]`                  	| Read n structs into a `Vec<_>` in the target field. The number of structs read is specified by the<br>byte count field. Note that the struct must have `byte_layout!{}` declared on it and<br>`#[derive(Default)]` as `parse_bytes::<_,_>(tail)` is called to a default instance of it.         	|
| Literal Composite Array     	| `composite_vec_lit [<target field> <int count> <struct name>]`                    	| Read n structs into a `Vec<_>` in the target field. The number of structs read is specified by the<br>literal integer provided. Note that the struct must have `byte_layout!{}` declared on it and<br>`#[derive(Default)]` as `parse_bytes::<_,_>(tail)` is called to a default instance of it. 	|

### Endianness

When specifying endianness, with a layout, there are three to choose from:

* `Big`
* `Small`
* `Native`

You will need to provide these according to the desired byte ordering. Note that single bytes (`u8`) do not need to provide it as it doesn't change the read.

## Walkthrough

Say we want to deserialise the following byte structure into a struct:

```
<2 bytes data field>
<1 byte (array data length: n)>
<n bytes array data>
<m bytes null terminated string>
```

First define a struct that you want to unpack, and derive `Default` for it.

```rust
#[derive(Default,Debug)]
pub struct ExampleStruct {
    pub field1: u16, // 2 bytes data field
    pub field2: u8,  // 1 byte array length
    pub field3: Vec<u16>, // n bytes array data
    pub field4: Vec<u8>, // Null terminated byte string
}
```

Then use the `byte_layout!{...}` macro to define the unpacking guidelines

```rust
byte_layout!{
    ExampleStruct
    value [field1, u16, Big]
    value [field2, u8]
    primitive_vec [field3, field2, u16, Small],
    bytes_vec_null_term [field4]
}
```

We can then parse it by calling `parse_bytes<I,E>(<bytes>)`:

```rust
let bytes: Vec<u8> = vec![
    0xDE, 0xAD,
    0x02,
    0x34, 0x12,
    0x78, 0x56,
    0xDE, 0xAD, 0xBE, 0xEF, 0x00
];

let example_struct: ExampleStruct = ExampleStruct::default();
match example_struct.parse_bytes<'_ &[u8], nom::error::Error<_>>(bytes.as_slice()) {
    Ok(()) => println!("Parsed: {:#04X?}", example_struct);
    Err(e) => println!("An error occured: {:?}", e);
}
```

We can see this results in the following output:

```
Parsed: ExampleStruct {
    field1: 0xDEAD,
    field2: 0x02,
    field3: [
        0x1234,
        0x5678,
    ],
    field4: [
        0xDE,
        0xAD,
        0xBE,
        0xEF
    ],
}
```

## Example

```rust
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
    /* Prints:
     * New: TestStruct {
     *     a: 0x00,
     *     b: 0x00,
     *     c: [],
     *     d: 0x00,
     *     e: [],
     *     g: 0x00,
     *     h: [],
     *     i: [],
     *     j: [],
     *     k: [],
     *     l: Other {
     *         f: 0x00,
     *     },
     *     m: [],
     * }
     */
    test_struct.parse_bytes::<&'_ [u8], nom::error::Error<_>>(test.as_slice());
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
     *     g: 0x02,
     *     h: [
     *         0x1234,
     *         0x5678,
     *     ],
     *     i: [
     *         0x01,
     *         0x10,
     *     ],
     *     j: [
     *         0xDEAD,
     *         0xBEEF,
     *     ],
     *     k: [
     *         Other {
     *             f: 0x43,
     *         },
     *         Other {
     *             f: 0x21,
     *         },
     *     ],
     *     l: Other {
     *         f: 0x55,
     *     },
     *     m: [
     *         0x01,
     *         0x02,
     *         0x03,
     *         0x04,
     *         0x05,
     *     ],
     * }
     */
}
```
