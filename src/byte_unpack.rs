pub trait ToVec<T> {
    fn to_vec(self) -> Vec<T>;
}

impl ToVec<u8> for &[u8] {
    fn to_vec(self) -> Vec<u8> {
        self.to_vec()
    }
}

#[derive(Debug)]
pub struct ByteLayoutParsingError {
    pub type_name: String,
    pub field_name: String,
}

impl fmt::Display for ByteLayoutParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Could not parse bytes into {}::{}",
            self.type_name,
            self.field_name,
        )
    }
}

#[macro_export]
macro_rules! byte_layout {
    (@inner value [$target_field:ident, $byte_parser:expr, $self_accessor:ident, $tail:ident]) => {
        match $byte_parser($tail) {
            Ok((t, b)) => {
                $tail = t;
                $self_accessor.$target_field = b;
            },
            Err(_) => return Err(ByteLayoutParsingError{
                type_name: std::any::type_name::<Self>().to_string(),
                field_name: stringify!($target_field).to_string(),
            }),
        };
    };
    (@inner pure_vec [$target_field_pure:ident, $ref_field_byte_count:ident, $self_accessor:ident, $tail:ident]) => {
        match nom::bytes::complete::take::<_, I, E>($self_accessor.$ref_field_byte_count)($tail) {
            Ok((t, b)) => {
                $tail = t;
                $self_accessor.$target_field_pure = b.to_vec();
            },
            Err(e) => return Err(ByteLayoutParsingError{
                type_name: std::any::type_name::<Self>().to_string(),
                field_name: stringify!($target_field_pure).to_string(),
            }),
        }
    };
    (@inner typed_vec [$target_field_composite:ident, $ref_field_composite_byte_count:ident, $composite_struct_name:ident, $self_accessor:ident, $tail:ident]) => {
        $self_accessor.$target_field_composite = Vec::with_capacity($self_accessor.$ref_field_composite_byte_count as usize);
        for _ in 0..$self_accessor.$ref_field_composite_byte_count {
            let mut other: $composite_struct_name = Default::default();
            match other.parse_bytes::<I,E>($tail) {
                Ok(new_tail) => {
                    $tail = new_tail;
                    $self_accessor.$target_field_composite.push(other);
                },
                Err(e) => return Err(e),
            };
        }
    };
    (
        $struct_name:ident
        $($alt:ident [$elem:ident$(, $args:tt)+])+
    ) => {
        impl $struct_name {
            #[allow(dead_code)]
            pub fn parse_bytes<I, E>(&mut self, bytes: I) -> Result<I, ByteLayoutParsingError>
            where
                I: nom::InputTake + crate::byte_unpack::ToVec<u8> + nom::Slice<std::ops::RangeFrom<usize>> + nom::InputIter<Item = u8> + nom::InputLength,
                E: nom::error::ParseError<I> {
                let mut tail = bytes;
                $(byte_layout!(@inner $alt [$elem$(, $args)+,self,tail]);)+
                return Ok(tail);
            }
        }
    }
}