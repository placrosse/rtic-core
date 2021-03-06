//! Core abstractions of the Real-Time Interrupt-driven Concurrency (RTIC) framework
//!
//! You can write generic *libraries* using the `Mutex` trait in this crate. If you want to write
//! application code then you'll need an *implementation* of the RTIC framework for a particular
//! architecture. Currently, there are implementations for these architectures and OSes:
//!
//! - [ARM Cortex-M](https://crates.io/crates/cortex-m-rtic)
// - [Linux]
// - [MSP430]
// - [RISC-V]

#![deny(missing_docs)]
#![deny(rust_2018_compatibility)]
#![deny(rust_2018_idioms)]
#![deny(warnings)]
#![no_std]

use core::ops;

/// Memory safe access to shared resources
///
/// In RTIC, locks are implemented as critical sections that prevent other tasks from *starting*.
/// These critical sections are implemented by temporarily increasing the dynamic priority of the
/// current context. Entering and leaving these critical sections is always done in bounded constant
/// time (a few instructions in bare metal contexts).
pub trait Mutex {
    /// Data protected by the mutex
    type T;

    /// Creates a critical section and grants temporary access to the protected data
    fn lock<R>(&mut self, f: impl FnOnce(&mut Self::T) -> R) -> R;
}

impl<'a, M> Mutex for &'a mut M
where
    M: Mutex,
{
    type T = M::T;

    fn lock<R>(&mut self, f: impl FnOnce(&mut M::T) -> R) -> R {
        M::lock(self, f)
    }
}

/// Newtype over `&'a mut T` that implements the `Mutex` trait
///
/// The `Mutex` implementation for this type is a no-op: no critical section is created
pub struct Exclusive<'a, T>(pub &'a mut T);

impl<'a, T> Mutex for Exclusive<'a, T> {
    type T = T;

    fn lock<R>(&mut self, f: impl FnOnce(&mut T) -> R) -> R {
        f(self.0)
    }
}

impl<'a, T> ops::Deref for Exclusive<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.0
    }
}

impl<'a, T> ops::DerefMut for Exclusive<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.0
    }
}
