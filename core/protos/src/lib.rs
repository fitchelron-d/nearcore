mod autogenerated;
pub use autogenerated::*;

pub use protobuf::Message;

#[cfg(feature = "with-serde")]
pub mod serde;