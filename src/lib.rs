#![no_std]

pub mod ser;

pub use ser::to_slice;
#[cfg(feature = "heapless")]
pub use ser::to_vec;
