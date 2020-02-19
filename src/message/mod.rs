#[macro_use]
mod macros;

pub mod basic;
pub mod special;

use std::convert::TryFrom;
use std::marker::PhantomData;

use self::basic::*;
use self::special::*;

pub enum MessageDecodeError {
    UnexpectedKind(Kind),
}

pub trait MessageEncoder {
    type Value;
    type Error;

    fn encode_field<'a, T>(&mut self, name: &'static str, value: &'a T) -> Result<(), Self::Error>
    where
        T: AsBasicTypeRef<'a, Self::Value>;
}

pub trait MessageDecoder {
    type Value;
    type Error: From<MessageDecodeError>;

    fn decode_field<T>(&mut self) -> Result<T, Self::Error>
    where
        T: TryFrom<BasicType<Self::Value>>;
}

pub trait Message<V>: Sized {
    /// Returns the message kind.
    fn kind(&self) -> KnownKind;

    /// Encodes the message given an encoder.
    fn encode<E>(&self, encoder: E) -> Result<(), E::Error>
    where
        E: MessageEncoder<Value = V>;

    /// Decodes the message given basic values and a kind.
    fn decode<D>(kind: Kind, decoder: D) -> Result<Self, D::Error>
    where
        D: MessageDecoder<Value = V>;
}

impl_all_standard_messages!(
    (
        GoodbyeMessage,
        Goodbye,
        [
            /// A URI uniquely describing the close reason.
            reason: Uri
        ]
    ),
    (
        HelloMessage,
        Hello,
        [
            /// The body of the message.
            body: Body<V>
        ]
    ),
    (
        ProveMessage,
        Prove,
        [
            /// The body of the message.
            body: Body<V>
        ]
    ),
    (
        ProofMessage,
        Proof,
        [
            /// The body of the message.
            body: Body<V>
        ]
    ),
    (
        ErrorMessage,
        Error,
        [
            /// The request kind that triggered the error.
            request_kind: Kind,
            /// The request ID that triggered the error.
            request_id: Id,
            /// A URI uniquely describing the error.
            error: Uri,
            /// Body of the error message.
            body: Body<V>
        ]
    ),
    (
        CancelMessage,
        Cancel,
        [
            /// The ID of the request we want to cancel.
            request_id: Id
        ]
    ),
    (
        CallMessage,
        Call,
        [
            /// An ID uniquely describing the call request.
            request_id: Id,
            /// A URI uniquely describing the procedure.
            procedure: Uri,
            /// The body of the message.
            body: Body<V>
        ]
    ),
    (
        ResultMessage,
        Result,
        [
            /// The ID of the call we are responding to.
            request_id: Id,
            /// The successful body result from the call.
            body: Body<V>
        ]
    ),
    (
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
    ),
    (
        PublishMessage,
        Publish,
        [
            /// An ID uniquely describing the publication request.
            request_id: Id,
            ///  A URI describing the topic we are publishing to.
            topic: Uri,
            /// The body of the event being published.
            body: Body<V>
        ]
    ),
    (
        PublishedMessage,
        Published,
        [
            /// The ID of the publication request.
            request_id: Id,
            /// An ID uniquely describing the publication.
            publication_id: Id
        ]
    ),
    (
        SubscribeMessage,
        Subscribe,
        [
            /// An ID uniquely describing the subscription request.
            request_id: Id,
            /// A URI describing the topic we are subscribing to.
            topic: Uri
        ]
    ),
    (
        SubscribedMessage,
        Subscribed,
        [
            /// The ID of the subscription request.
            request_id: Id,
            /// An ID uniquely describing the subscription.
            subscription_id: Id
        ]
    ),
    (
        UnsubscribeMessage,
        Unsubscribe,
        [
            /// An ID uniquely describing the unsubscribe request.
            request_id: Id,
            /// The subscription ID we are unsubscribing from.
            subscription_id: Id
        ]
    ),
    (
        UnsubscribedMessage,
        Unsubscribed,
        [
            /// The ID of the unsubscribe request.
            request_id: Id
        ]
    )
);
