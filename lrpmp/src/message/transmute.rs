use std::collections::VecDeque;
use std::marker::PhantomData;

use super::dec::{ArrayFieldDecoder, KindDecoder};
use super::enc::{MessageEncoder, MessageFieldEncoder};
use super::{Message, MessageError};
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
