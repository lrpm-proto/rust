macro_rules! impl_invalid_basic_types {
    ($($ty:ident),*) => {
        $(
            impl_invalid_basic_type!($ty);
        )*
    };
}

macro_rules! impl_invalid_basic_type {
    (U8) => {
        #[inline]
        fn as_u8(&self) -> u8 {
            panic_with_expected_type::<Self>(&self, BasicType::U8)
        }
    };
    (U64) => {
        #[inline]
        fn as_u64(&self) -> u64 {
            panic_with_expected_type::<Self>(&self, BasicType::U64)
        }
    };
    (Str) => {
        #[inline]
        fn as_str(&self) -> &str {
            panic_with_expected_type::<Self>(&self, BasicType::Str)
        }

        #[inline]
        fn into_string(self) -> ByteString {
            panic_with_expected_type::<Self>(&self, BasicType::Str)
        }
    };
    (Map) => {
        #[inline]
        fn as_map(&self) -> &Self::Map {
            panic_with_expected_type::<Self>(&self, BasicType::Map)
        }

        #[inline]
        fn into_map(self) -> Self::Map {
            panic_with_expected_type::<Self>(&self, BasicType::Map)
        }
    };
    (Val) => {
        #[inline]
        fn as_val(&self) -> &Self::Val {
            panic_with_expected_type::<Self>(&self, BasicType::Val)
        }

        #[inline]
        fn into_val(self) -> Self::Val {
            panic_with_expected_type::<Self>(&self, BasicType::Val)
        }
    };
}
