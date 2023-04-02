#![no_std]

pub mod ser;

#[doc(inline)]
pub use ser::to_slice;
#[cfg(feature = "heapless")]
#[doc(inline)]
pub use ser::to_vec;
