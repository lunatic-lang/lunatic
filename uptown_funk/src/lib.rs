pub mod memory;
pub mod state;
pub mod types;

use std::convert::Into;
use std::fmt::Debug;

pub use smallvec::SmallVec;
pub use types::{FromWasm, ToWasm};
pub use uptown_funk_macro::host_functions;

/// Provides access to the instance execution environment.
pub trait Executor {
    /// Execute `Future` f.
    #[cfg(feature = "async")]
    fn async_<R, F>(&self, f: F) -> R
    where
        F: std::future::Future<Output = R>;

    /// Get mutable access to the instance memory.
    fn memory(&self) -> memory::Memory;
}

pub trait HostFunctions: Sized {
    type Return;
    type Wrap;

    fn split(self) -> (Self::Return, Self::Wrap);

    fn add_to_linker<E>(api: Self::Wrap, executor: E, linker: &mut wasmtime::Linker)
    where
        E: Executor + Clone + 'static;
}

pub struct Trap {
    message: String,
}

impl Debug for Trap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.message, f)
    }
}

impl Trap {
    pub fn new<I: Into<String>>(message: I) -> Self {
        Self {
            message: message.into(),
        }
    }

    pub fn try_option<R: Debug>(result: Option<R>) -> Result<R, Trap> {
        match result {
            Some(r) => Ok(r),
            None => Err(Trap::new(
                "Host function trapped: Memory location not inside wasm guest",
            )),
        }
    }

    pub fn try_result<R: Debug, E: Debug>(result: Result<R, E>) -> Result<R, Trap> {
        match result {
            Ok(r) => Ok(r),
            Err(_) => {
                let message = format!("Host function trapped: {:?}", result);
                Err(Trap::new(message))
            }
        }
    }
}

impl<S> ToWasm<S> for Trap {
    type To = ();

    fn to(_: S, _: &impl Executor, v: Self) -> Result<(), Trap> {
        Err(v)
    }
}

impl From<Trap> for wasmtime::Trap {
    fn from(trap: Trap) -> Self {
        wasmtime::Trap::new(trap.message)
    }
}

impl<S> From<std::sync::PoisonError<S>> for Trap {
    fn from(_: std::sync::PoisonError<S>) -> Self {
        Trap::new("Poison error accessing state")
    }
}

#[repr(C)]
pub struct IoVecT {
    pub ptr: u32,
    pub len: u32,
}
