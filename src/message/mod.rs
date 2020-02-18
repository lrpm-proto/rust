pub mod basic;
pub mod special;

use std::marker::PhantomData;

use self::basic::*;
use self::special::*;

pub trait MessageEncoder {
    type Value;
    type Error;

    fn encode_field<'a, T>(&mut self, name: &'static str, value: &'a T) -> Result<(), Self::Error>
    where
        T: AsBasicTypeRef<'a, Self::Value>;
}

pub trait Message<V> {
    fn kind_str(&self) -> &'static str;

    fn encode<E>(&self, encoder: E) -> Result<(), E::Error>
    where
        E: MessageEncoder<Value = V>;
}

macro_rules! impl_standard_message {
    (
        $name:ident,
        $kind:ident,
        [ $($field:ident: $field_ty:ty),* ]
    ) => {
        #[derive(Debug, Clone)]
        pub struct $name<'a, V> {
            $(pub $field: $field_ty),*,
            pub meta: Meta<V>,
            _marker: PhantomData::<&'a ()>,
        }

        impl<'a, V> Message<V> for $name<'a, V> {
            fn kind_str(&self) -> &'static str {
                StandardKind::$kind.to_str()
            }

            fn encode<E>(&self, mut encoder: E) -> Result<(), E::Error>
            where
                E: MessageEncoder<Value = V>,
            {
                $(
                    encoder.encode_field(
                        stringify!($field),
                        &self.$field
                    )?;
                )*
                Ok(())
            }
        }
    };
}

#[derive(Debug, Clone)]
pub enum StandardMessage<'a, V> {
    Goodbye(GoodbyeMessage<'a, V>),
    Hello(HelloMessage<'a, V>),
    Prove(ProveMessage<'a, V>),
    Proof(ProofMessage<'a, V>),
    Error(ErrorMessage<'a, V>),
    Cancel(CancelMessage<'a, V>),
    Call(CallMessage<'a, V>),
    Result(ResultMessage<'a, V>),
    Event(EventMessage<'a, V>),
    Publish(PublishMessage<'a, V>),
    Published(PublishedMessage<'a, V>),
    Subscribe(SubscribeMessage<'a, V>),
    Subscribed(SubscribedMessage<'a, V>),
    Unsubscribe(UnsubscribeMessage<'a, V>),
    Unsubscribed(UnsubscribedMessage<'a, V>),
}

impl_standard_message!(GoodbyeMessage, Goodbye, [reason: Uri]);

impl_standard_message!(HelloMessage, Hello, [body: Body<V>]);

impl_standard_message!(ProveMessage, Prove, [body: Body<V>]);

impl_standard_message!(ProofMessage, Proof, [body: Body<V>]);

impl_standard_message!(
    ErrorMessage,
    Error,
    [
        request_kind: Kind,
        request_id: Id,
        error: UriRef<'a>,
        body: Body<V>
    ]
);

impl_standard_message!(CancelMessage, Cancel, [request_id: Id]);

impl_standard_message!(
    CallMessage,
    Call,
    [request_id: Id, procedure: UriRef<'a>, body: Body<V>]
);

impl_standard_message!(ResultMessage, Result, [request_id: Id, body: Body<V>]);

impl_standard_message!(
    EventMessage,
    Event,
    [publication_id: Id, subscription_id: Id, body: Body<V>]
);

impl_standard_message!(
    PublishMessage,
    Publish,
    [request_id: Id, topic: UriRef<'a>, body: Body<V>]
);

impl_standard_message!(
    PublishedMessage,
    Published,
    [request_id: Id, publication_id: Id]
);

impl_standard_message!(SubscribeMessage, Subscribe, [request_id: Id, topic: Uri]);

impl_standard_message!(
    SubscribedMessage,
    Subscribed,
    [request_id: Id, subscription_id: Id]
);

impl_standard_message!(
    UnsubscribeMessage,
    Unsubscribe,
    [request_id: Id, subscription_id: Id]
);

impl_standard_message!(UnsubscribedMessage, Unsubscribed, [request_id: Id]);
