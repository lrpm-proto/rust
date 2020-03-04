pub mod codec;
pub mod io;
pub mod message;
pub mod serde;
pub mod types;

pub(crate) mod std_impl {
    use crate::message::dec::*;
    use crate::message::enc::*;
    use crate::message::*;
    use crate::types::*;

    ::lrpmp_macros::impl_std_messages!();
}
