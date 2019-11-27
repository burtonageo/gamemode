use crate::{Status, Error};
use libc::pid_t;
use std::fmt;

pub struct GameMode {
    _priv: (),
}

impl GameMode {
    #[inline]
    pub fn start() -> Result<Self, Error> {
        Err(Error::UnsupportedPlatform)
    }

    #[inline]
    pub fn start_for(pid: pid_t) -> Result<Self, Error> {
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
