use std::sync::Arc;
use std::marker::PhantomData;

use super::transport::Transport;

struct HubInner<T, V> {
    value: PhantomData::<V>,
    transport: T,
}

pub struct Hub<T, V> {
    inner: Arc<HubInner<T, V>>,
}

impl<T, V> Hub<T, V>
where
    T: Transport<V>,
{
    pub fn new(transport: T) -> Self {
        let inner = HubInner {
            transport,
            value: PhantomData,
        };
        Self {
            inner: Arc::new(inner),
        }
    }
}
