use super::message::BusMessage;

use futures::{Sink, Stream};

pub trait Read<V>: Stream<Item = BusMessage<V>> {}

pub trait Write<V>: Sink<BusMessage<V>> {}

pub trait Transport<V>: Read<V> + Write<V> {}

impl<T, V> Transport<V> for T where T: Read<V> + Write<V> {}
