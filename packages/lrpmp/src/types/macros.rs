macro_rules! impl_invalid_basic_types {
    (<$M:ty, $V:ty> $($ty:ident),*) => {
        $(
            impl_invalid_basic_type!($ty, $M, $V);
        )*
    };
}

macro_rules! impl_invalid_basic_type {
    (U8, $M:ty, $V:ty) => {
        #[inline]
        fn as_u8(&self) -> u8 {
            <Self as BasicValueExt<$M, $V>>::assert_is_type(self, BasicType::U8);
            unreachable!()
        }
    };
    (U64, $M:ty, $V:ty) => {
        #[inline]
        fn as_u64(&self) -> u64 {
            <Self as BasicValueExt<$M, $V>>::assert_is_type(self, BasicType::U64);
            unreachable!()
        }
    };
    (Str, $M:ty, $V:ty) => {
        #[inline]
        fn as_str(&self) -> &str {
            <Self as BasicValueExt<$M, $V>>::assert_is_type(self, BasicType::Str);
            unreachable!()
        }

        #[inline]
        fn into_string(self) -> String {
            <Self as BasicValueExt<$M, $V>>::assert_is_type(&self, BasicType::Str);
            unreachable!()
        }
    };
    (Map, $M:ty, $V:ty) => {
        #[inline]
        fn as_map(&self) -> &$M {
            <Self as BasicValueExt<$M, $V>>::assert_is_type(self, BasicType::Map);
            unreachable!()
        }

        #[inline]
        fn into_map(self) -> $M {
            <Self as BasicValueExt<$M, $V>>::assert_is_type(&self, BasicType::Map);
            unreachable!()
        }
    };
    (Val, $M:ty, $V:ty) => {
        #[inline]
        fn as_val(&self) -> &$V {
            <Self as BasicValueExt<$M, $V>>::assert_is_type(self, BasicType::Val);
            unreachable!()
        }

        #[inline]
        fn into_val(self) -> $V {
            <Self as BasicValueExt<$M, $V>>::assert_is_type(&self, BasicType::Val);
            unreachable!()
        }
    };
}
