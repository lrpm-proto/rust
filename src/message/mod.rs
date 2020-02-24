#[macro_use]
mod macros;
mod encdec;

pub mod basic;
pub mod special;

use self::basic::*;
use self::special::*;

pub use self::encdec::*;

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
        /// `GOODBYE`
        GoodbyeMessage,
        Goodbye,
        [
            /// A URI uniquely describing the close reason.
            reason: Uri
        ]
    ),
    (
        /// `HELLO`
        HelloMessage,
        Hello,
        [
            /// The body of the message.
            body: Body<V>
        ]
    ),
    (
        /// `PROVE`
        ProveMessage,
        Prove,
        [
            /// The body of the message.
            body: Body<V>
        ]
    ),
    (
        /// `PROOF`
        ProofMessage,
        Proof,
        [
            /// The body of the message.
            body: Body<V>
        ]
    ),
    (
        /// `ERROR`
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
        /// `CANCEL`
        CancelMessage,
        Cancel,
        [
            /// The ID of the request we want to cancel.
            request_id: Id
        ]
    ),
    (
        /// `CALL`
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
        /// `RESULT`
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
        /// `EVENT`
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
        /// `PUBLISH`
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
        /// `PUBLISHED`
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
        /// `SUBSCRIBE`
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
        /// `SUBSCRIBED`
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
        /// `UNSUBSCRIBE`
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
        /// `UNSUBSCRIBED`
        UnsubscribedMessage,
        Unsubscribed,
        [
            /// The ID of the unsubscribe request.
            request_id: Id
        ]
    )
);
