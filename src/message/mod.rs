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
    fn kind(&self) -> KnownKind;

    fn encode<E>(&self, encoder: E) -> Result<(), E::Error>
    where
        E: MessageEncoder<Value = V>;
}

macro_rules! impl_standard_message {
    (
        $name:ident,
        $kind:ident,
        [ $(
            $(#[$field_attr:meta])*
            $field:ident: $field_ty:ty
        ),* ]
    ) => {
        #[derive(Debug, Clone)]
        pub struct $name<'a, V> {
            $(
                $(#[$field_attr])*
                pub $field: $field_ty
            ),*,
            #[doc="Optional meta information on this message."]
            pub meta: Meta<V>,
            _marker: PhantomData::<&'a ()>,
        }

        impl<'a, V> $name<'a, V> {
            pub fn new(
                $($field: $field_ty),*,
                meta: Meta<V>,
            ) -> Self {
                Self {
                    $($field),*,
                    meta,
                    _marker: PhantomData,
                }
            }
        }

        impl<'a, V> Message<V> for $name<'a, V> {
            /// Returns the message kind.
            fn kind(&self) -> KnownKind {
                KnownKind::Standard(StandardKind::$kind)
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

impl_standard_message!(
    GoodbyeMessage,
    Goodbye,
    [
        /// A URI uniquely describing the close reason.
        reason: Uri<'a>
    ]
);

impl_standard_message!(
    HelloMessage,
    Hello,
    [
        /// The body of the message.
        body: Body<V>
    ]
);

impl_standard_message!(
    ProveMessage,
    Prove,
    [
        /// The body of the message.
        body: Body<V>
    ]
);

impl_standard_message!(
    ProofMessage,
    Proof,
    [
        /// The body of the message.
        body: Body<V>
    ]
);

impl_standard_message!(
    ErrorMessage,
    Error,
    [
        /// The request kind that triggered the error.
        request_kind: Kind,
        /// The request ID that triggered the error.
        request_id: Id,
        /// A URI uniquely describing the error.
        error: Uri<'a>,
        /// Body of the error message.
        body: Body<V>
    ]
);

impl_standard_message!(
    CancelMessage,
    Cancel,
    [
        /// The ID of the request we want to cancel.
        request_id: Id
    ]
);

impl_standard_message!(
    CallMessage,
    Call,
    [
        /// An ID uniquely describing the call request.
        request_id: Id,
        /// A URI uniquely describing the procedure.
        procedure: Uri<'a>,
        /// The body of the message.
        body: Body<V>
    ]
);

impl_standard_message!(
    ResultMessage,
    Result,
    [
        /// The ID of the call we are responding to.
        request_id: Id,
        /// The successful body result from the call.
        body: Body<V>
    ]
);

impl_standard_message!(
    EventMessage,
    Event,
    [
        /// The publication ID.
        publication_id: Id,
        /// The subscription ID.
        subscription_id: Id,
        /// The body of the event.
        body: Body<V>
    ]
);

impl_standard_message!(
    PublishMessage,
    Publish,
    [
        /// An ID uniquely describing the publication request.
        request_id: Id,
        ///  A URI describing the topic we are publishing to.
        topic: Uri<'a>,
        /// The body of the event being published.
        body: Body<V>
    ]
);

impl_standard_message!(
    PublishedMessage,
    Published,
    [
        /// The ID of the publication request.
        request_id: Id,
        /// An ID uniquely describing the publication.
        publication_id: Id
    ]
);

impl_standard_message!(
    SubscribeMessage,
    Subscribe,
    [
        /// An ID uniquely describing the subscription request.
        request_id: Id,
        /// A URI describing the topic we are subscribing to.
        topic: Uri<'a>
    ]
);

impl_standard_message!(
    SubscribedMessage,
    Subscribed,
    [
        /// The ID of the subscription request.
        request_id: Id,
        /// An ID uniquely describing the subscription.
        subscription_id: Id
    ]
);

impl_standard_message!(
    UnsubscribeMessage,
    Unsubscribe,
    [
        /// An ID uniquely describing the unsubscribe request.
        request_id: Id,
        /// The subscription ID we are unsubscribing from.
        subscription_id: Id
    ]
);

impl_standard_message!(
    UnsubscribedMessage,
    Unsubscribed,
    [
        /// The ID of the unsubscribe request.
        request_id: Id
    ]
);
