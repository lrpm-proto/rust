use proc_macro_hack::proc_macro_hack;

//pub mod bus;
pub mod codec;
pub mod io;
pub mod message;
pub mod serde;
pub mod types;

/// Returns a valid URI given a static str.
///
/// # Example
/// ```rust
/// use lrpmp::uri;
/// use lrpmp::types::Uri;
///
/// const MY_URI: Uri = uri!("hello.world");
/// ```
#[proc_macro_hack]
pub use ::lrpmp_macros::uri;

pub(crate) mod std_impl {
    use crate::message::dec::*;
    use crate::message::enc::*;
    use crate::message::*;
    use crate::types::*;

    ::lrpmp_macros::impl_std_messages!();
}
