mod hub;
mod message;
mod client;
mod transport;

// pub use self::hub::Hub;
// pub use self::client::Client;
// pub use self::transport::*;
// pub use self::message::BusMessage;

use crate::codec::generic::Meta;
use crate::types::Uri;

pub enum Error<V> {
    Remote(RemoteError<V>),
}

pub struct RemoteError<V> {
    error: Uri,
    body: V,
    meta: Meta<V>,
}
