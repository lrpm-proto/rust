#[macro_use]
mod macros;
mod codec;
mod error;
mod io;

pub mod basic;
pub mod special;

use self::basic::*;
use self::special::*;

pub use self::codec::*;
pub use self::error::*;
pub use self::io::*;

pub trait Message<V>: Sized {
    /// Returns the message kind.
    fn kind(&self) -> KnownKind;

    /// Encodes the message given an encoder.
    fn encode<E>(self, encoder: E) -> Result<(), MessageError<E::Error>>
    where
        E: MessageEncoder<V>;

    /// Decodes the message given basic values and a known kind.
    fn decode<D>(kind: KnownKind, decoder: D) -> Result<Self, MessageError<D::Error>>
    where
        D: MessageDecoder<V>;

    /// Returns the lower and upper bound of the number of fields in the message.
    fn field_count(&self) -> (usize, Option<usize>);

    // /// Convert the message into a standard message if applicable.
    // fn into_standard(self) -> Result<StandardMessage<V>, MessageError<()>>;
}

#[derive(Debug, Clone)]
pub struct GenericMessage<V> {
    kind: KnownKind,
    fields: Vec<BasicValue<V>>,
}

impl<V> GenericMessage<V> {
    pub fn new(kind: KnownKind, fields: Vec<BasicValue<V>>) -> Self {
        Self { kind, fields }
    }
}

impl<V> Message<V> for GenericMessage<V> {
    fn kind(&self) -> KnownKind {
        self.kind
    }

    fn encode<E>(self, encoder: E) -> Result<(), MessageError<E::Error>>
    where
        E: MessageEncoder<V>,
    {
        let mut encoder = encoder.for_message(&self)?;
        for field in self.fields.into_iter() {
            encoder.encode_field(None, field)?;
        }
        Ok(())
    }

    fn decode<D>(kind: KnownKind, decoder: D) -> Result<Self, MessageError<D::Error>>
    where
        D: MessageDecoder<V>,
    {
        let mut decoder = decoder.for_message(kind)?;
        let cap = decoder.remaining().unwrap_or(0);
        let mut fields = Vec::with_capacity(cap);
        while Some(0) != decoder.remaining() {
            fields.push(decoder.decode_field(None)?);
        }
        Ok(Self::new(kind, fields))
    }

    fn field_count(&self) -> (usize, Option<usize>) {
        let len = 1 + self.fields.len();
        (len, Some(len))
    }

    // fn into_standard(self) -> Result<StandardMessage<V>, MessageError<()>> {
    //     unimplemented!()
    // }
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
