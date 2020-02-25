#[macro_use]
mod macros;
mod encdec;
mod error;
mod io;

pub mod basic;
pub mod special;

use self::basic::*;
use self::special::*;

pub use self::encdec::*;
pub use self::error::*;
pub use self::io::*;

pub trait Message<V>: Sized {
    /// Returns the message kind.
    fn kind(&self) -> KnownKind;

    /// Consumes and encodes the message given an encoder.
    fn encode<E>(self, encoder: E) -> Result<E::Ok, MessageError<E::Error>>
    where
        E: MessageEncoder<V>,
    {
        self.encode_ref(encoder)
    }

    /// Encodes the message given an encoder.
    fn encode_ref<E>(&self, encoder: E) -> Result<E::Ok, MessageError<E::Error>>
    where
        E: MessageEncoder<V>;

    /// Decodes the message given basic values and a known kind.
    fn decode<D>(decoder: D) -> Result<Self, MessageError<D::Error>>
    where
        D: MessageDecoder<V>;

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

    fn encode<E>(self, encoder: E) -> Result<E::Ok, MessageError<E::Error>>
    where
        E: MessageEncoder<V>,
    {
        let mut encoder = encoder.start(self.kind())?;
        for field in self.fields.into_iter() {
            encoder.encode_field(None, field)?;
        }
        encoder.end()
    }

    fn encode_ref<E>(&self, encoder: E) -> Result<E::Ok, MessageError<E::Error>>
    where
        E: MessageEncoder<V>,
    {
        let mut encoder = encoder.start(self.kind())?;
        for field in self.fields.iter() {
            encoder.encode_field_ref(None, field)?;
        }
        encoder.end()
    }

    fn decode<D>(decoder: D) -> Result<Self, MessageError<D::Error>>
    where
        D: MessageDecoder<V>,
    {
        let (kind, mut decoder) = decoder.start()?;
        let cap = decoder.remaining().unwrap_or(0);
        let mut fields = Vec::with_capacity(cap);
        while Some(0) != decoder.remaining() {
            fields.push(decoder.decode_field(None)?);
        }
        Ok(Self::new(kind, fields))
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
            reason: Uri,
            /// Optional meta information on this message.
            meta: Meta<V>
        ]
    ),
    (
        /// `HELLO`
        HelloMessage,
        Hello,
        [
            /// The body of the message.
            body: Body<V>,
            /// Optional meta information on this message.
            meta: Meta<V>
        ]
    ),
    (
        /// `PROVE`
        ProveMessage,
        Prove,
        [
            /// The body of the message.
            body: Body<V>,
            /// Optional meta information on this message.
            meta: Meta<V>
        ]
    ),
    (
        /// `PROOF`
        ProofMessage,
        Proof,
        [
            /// The body of the message.
            body: Body<V>,
            /// Optional meta information on this message.
            meta: Meta<V>
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
            body: Body<V>,
            /// Optional meta information on this message.
            meta: Meta<V>
        ]
    ),
    (
        /// `CANCEL`
        CancelMessage,
        Cancel,
        [
            /// The ID of the request we want to cancel.
            request_id: Id,
            /// Optional meta information on this message.
            meta: Meta<V>
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
            body: Body<V>,
            /// Optional meta information on this message.
            meta: Meta<V>
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
            body: Body<V>,
            /// Optional meta information on this message.
            meta: Meta<V>
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
            body: Body<V>,
            /// Optional meta information on this message.
            meta: Meta<V>
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
            body: Body<V>,
            /// Optional meta information on this message.
            meta: Meta<V>
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
            publication_id: Id,
            /// Optional meta information on this message.
            meta: Meta<V>
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
            topic: Uri,
            /// Optional meta information on this message.
            meta: Meta<V>
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
            subscription_id: Id,
            /// Optional meta information on this message.
            meta: Meta<V>
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
            subscription_id: Id,
            /// Optional meta information on this message.
            meta: Meta<V>
        ]
    ),
    (
        /// `UNSUBSCRIBED`
        UnsubscribedMessage,
        Unsubscribed,
        [
            /// The ID of the unsubscribe request.
            request_id: Id,
            /// Optional meta information on this message.
            meta: Meta<V>
        ]
    )
);
