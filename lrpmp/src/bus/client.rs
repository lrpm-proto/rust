use std::future::Future;

use super::{Error, Meta, Uri};

pub trait RpcClient<V> {
    type Future: Future<Output = Result<(V, Meta<V>), Error<V>>>;

    #[inline]
    fn call(&mut self, procedure: &Uri, body: V) -> Self::Future {
        self.call_with_meta(procedure, body, Meta::default())
    }

    fn call_with_meta(&mut self, procedure: &Uri, body: V, meta: Meta<V>) -> Self::Future;
}
