use crate::{Error, Status};
use std::fmt;

/// The main struct which holds the `GameMode` state.
///
/// On non-Linux operating systems, this type does nothing.
pub struct GameMode {
    _priv: (),
}

impl GameMode {
    #[inline]
    pub fn start() -> Result<Self, Error> {
        Err(Error::UnsupportedPlatform)
    }

    #[rustfmt::skip]
    #[inline]
    pub fn start_for(
        #[cfg(windows)]
        pid: libc::c_int,

        #[cfg(unix)]
        pid: libc::pid_t,
    ) -> Result<Self, Error> {
        let _ = pid;
        Err(Error::UnsupportedPlatform)
    }

    #[inline]
    pub fn status(&self) -> Result<Status, Error> {
        Err(Error::UnsupportedPlatform)
    }
}

impl fmt::Debug for GameMode {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("GameMode { .. }")
    }
}
