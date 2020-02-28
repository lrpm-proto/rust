impl_standard_kind!(
    // Init
    (Goodbye, "GOODBYE", 1, 2),
    (Hello, "HELLO", 2, 2),
    (Prove, "PROVE", 3, 2),
    (Proof, "PROOF", 4, 2),
    // Generic
    (Error, "ERROR", 20, 5),
    (Cancel, "CANCEL", 21, 2),
    // RPC
    (Call, "CALL", 40, 4),
    (Result, "RESULT", 41, 3),
    // PubSub
    (Event, "EVENT", 60, 4),
    (Publish, "PUBLISH", 61, 4),
    (Published, "PUBLISHED", 62, 3),
    (Subscribe, "SUBSCRIBE", 63, 3),
    (Subscribed, "SUBSCRIBED", 64, 3),
    (Unsubscribe, "UNSUBSCRIBE", 65, 3),
    (Unsubscribed, "UNSUBSCRIBED", 66, 2)
);

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
