//! The bytecode circuit implementation.

/// Bytecode unroller
pub mod bytecode_unroller;
/// Bytecode circuit tester
#[cfg(any(feature = "test", test))]
pub mod dev;
pub(crate) mod param;
