//! # GameMode
//!
//! `GameMode` is a small crate which allows programs to interact
//! with the gamemode daemon.
//!
//! This daemon allows certain optimizations and settings to be automatically applied
//! on linux while the game is playing. See the [`github`] page for more info.
//!
//! On non-Linux systems, the functionality is stubbed out and all functions will
//! return an [`UnsupportedPlatform`] error.
//!
//! [`github`]: https://github.com/FeralInteractive/gamemode
//! [`UnsupportedPlatform`]: enum.Error.html#variant.UnsupportedPlatform

#[cfg(target_os = "linux")]
#[path = "linux_imp.rs"]
mod imp;

#[cfg(not(target_os = "linux"))]
#[path = "not_linux_imp.rs"]
mod imp;

pub use imp::GameMode;

use std::{error::Error as StdError, fmt};

/// Possible status values which can be obtained
/// when querying the game mode status of the current
/// game.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Status {
    /// Gamemode is currently active.
    Active,
    /// Gamemode is currently active, and this client is
    /// registered with gamemode.
    Registered,
    /// Gamemode is inactive.
    Inactive,
}

/// Error types which can result from gamemode.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Error {
    /// The platform is not supported. This error
    /// can be safely ignored.
    UnsupportedPlatform,
    /// An error occurred in libgamemode.
    OperationFailed {
        /// The reason for the failure.
        reason: String,
    },
}

impl fmt::Display for Error {
    #[allow(deprecated)]
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(StdError::description(self))
    }
}

impl StdError for Error {
    #[inline]
    fn description(&self) -> &str {
        match *self {
            Error::OperationFailed { ref reason } => &reason,
            Error::UnsupportedPlatform => "GameMode is not supported on this platform",
        }
    }
}
