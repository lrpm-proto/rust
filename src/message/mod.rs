#[macro_use]
mod macros;
mod codec;

pub mod basic;
pub mod special;

use self::basic::*;
use self::special::*;

pub use self::codec::*;

pub trait Message<V>: Sized {
    /// Returns the message kind.
    fn kind(&self) -> KnownKind;

    /// Encodes the message given an encoder.
    fn encode<C, E>(&self, encoder: E) -> Result<(), C::Error>
    where
        C: MessageCodec<Value = V>,
        E: MessageEncoder<C>;

    /// Decodes the message given basic values and a kind.
    fn decode<'de, C, D>(kind: Kind, decoder: D) -> Result<Self, MessageDecodeError<C::Error>>
    where
        C: MessageCodec<Value = V>,
        D: MessageDecoder<'de, C>;

    /// Returns the lower and upper bound of the number of fields in the message.
    fn field_count(&self) -> (usize, Option<usize>);

    /// Convert the message into a standard message if applicable.
    fn into_standard(self) -> Option<StandardMessage<V>>;
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
