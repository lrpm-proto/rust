use std::collections::VecDeque;
use std::marker::PhantomData;

use super::{
    ArrayFieldDecoder, KindDecoder, Message, MessageEncoder, MessageError, MessageFieldEncoder,
};
use crate::types::{BasicValue, BasicValueExt, ConcreteBasicValue, KnownKind};

pub(crate) struct TransmuteEncoder<M, MO, VO>
where
    M: Message<MO, VO>,
{
    out: PhantomData<(M, MO, VO)>,
}

impl<M, MO, VO> TransmuteEncoder<M, MO, VO>
where
    M: Message<MO, VO>,
{
    pub fn new() -> Self {
        Self { out: PhantomData }
    }
}

impl<M, MI, VI, MO, VO> MessageEncoder<MI, VI> for TransmuteEncoder<M, MO, VO>
where
    M: Message<MO, VO>,
    MO: From<MI>,
    VO: From<VI>,
{
    type Ok = M;
    type Error = ();
    type FieldEncoder = TransmuteFieldEncoder<M, MO, VO>;

    fn start(self, kind: KnownKind) -> Result<Self::FieldEncoder, MessageError<Self::Error>> {
        let fields = VecDeque::with_capacity(kind.field_count().1.unwrap_or(0));
        Ok(TransmuteFieldEncoder {
            kind,
            fields,
            out: PhantomData,
        })
    }
}

pub(crate) struct TransmuteFieldEncoder<M, MO, VO>
where
    M: Message<MO, VO>,
{
    kind: KnownKind,
    fields: VecDeque<ConcreteBasicValue<MO, VO>>,
    out: PhantomData<M>,
}

impl<M, MI, VI, MO, VO> MessageFieldEncoder<MI, VI> for TransmuteFieldEncoder<M, MO, VO>
where
    M: Message<MO, VO>,
    MO: From<MI>,
    VO: From<VI>,
{
    type Ok = M;
    type Error = ();

    fn encode_field<F>(
        &mut self,
        _name: Option<&'static str>,
        value: F,
    ) -> Result<(), MessageError<Self::Error>>
    where
        F: BasicValue<MI, VI>,
    {
        self.fields.push_back(value.map_into()?);
        Ok(())
    }

    fn encode_field_ref<F>(
        &mut self,
        _name: Option<&'static str>,
        _value: &F,
    ) -> Result<(), MessageError<Self::Error>>
    where
        F: BasicValue<MI, VI>,
    {
        unimplemented!()
    }

    fn end(self) -> Result<Self::Ok, MessageError<Self::Error>> {
        let field_decoder = ArrayFieldDecoder::new(self.fields);
        let kind_decoder = KindDecoder::new(self.kind, field_decoder);
        M::decode(kind_decoder)
    }
}

// struct TransmuteDecoder<MO, VO>
// where
//     M: Message<MO, VO>,
// {
//     out: PhantomData<M>,
//     kind: KnownKind,
//     fields: Vec<ConcreteBasicValue<MO, VO>>,
// }

// impl<M, MO, VO> MessageDecoder<MO, VO> for TransmuteDecoder<M, MO, VO>
// where
//     M: Message<MO, VO>,
// {
//     type Error = ();
//     type FieldDecoder = ArrayFieldDecoder<MO, VO>;

//     fn start(self) -> Result<(KnownKind, Self::FieldDecoder), MessageError<Self::Error>> {
//         let kind = self.kind;
//         let decoder = TransmuteFieldDecoder {
//             out: self.out,
//             fields: self.fields,
//         };
//         Ok((kind, decoder))
//     }
// }

// struct TransmuteFieldDecoder<M, MO, VO>
// where
//     M: Message<MO, VO>,
// {
//     out: PhantomData<M>,
//     fields: Vec<ConcreteBasicValue<MO, VO>>,
// }

// impl<M, V> MessageFieldDecoder<M, V> for TransmuteFieldDecoder<M, V> {
//     type Error = ();

//     fn remaining(&self) -> Option<usize> {
//         Some(self.fields.len())
//     }

//     fn decode_field<T>(
//         &mut self,
//         name: Option<&'static str>,
//     ) -> Result<T, MessageError<Self::Error>>
//     where
//         T: FromBasicValuePart<M, V>,
//         T::Error: Into<MessageError<Self::Error>>
//     {
//         T::fro
//     }
// }
