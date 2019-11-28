use crate::{Error, Status};
use gamemode_sys::{
    wrapper_gamemode_error_string, wrapper_gamemode_query_status,
    wrapper_gamemode_query_status_for, wrapper_gamemode_request_end,
    wrapper_gamemode_request_end_for, wrapper_gamemode_request_start,
    wrapper_gamemode_request_start_for,
};
use libc::pid_t;
use std::{ffi::CStr, fmt, mem, num::NonZeroI32, thread};

pub struct GameMode {
    active_pid: Option<NonZeroI32>,
}

impl GameMode {
    #[inline]
    pub fn start() -> Result<Self, Error> {
        let result = unsafe { wrapper_gamemode_request_start() };

        if result < 0 {
            Err(error())
        } else {
            Ok(GameMode { active_pid: None })
        }
    }

    #[inline]
    pub fn start_for(pid: pid_t) -> Result<Self, Error> {
        let result = unsafe { wrapper_gamemode_request_start_for(pid) };

        if result < 0 {
            Err(error())
        } else {
            NonZeroI32::new(pid)
                .map(|pid| GameMode { active_pid: pid })
                .ok_or_else(|| {
                    Error::OperationFailed("Tried to start game mode for pid 0".to_string())
                })?;
        }
    }

    #[inline]
    pub fn status(&self) -> Result<Status, Error> {
        let to_status = |num| match num {
            0 => Status::Inactive,
            1 => Status::Active,
            2 => Status::Registered,
            _ => unreachable!(),
        };

        match *self.active_pid {
            None => to_status(wrapper_gamemode_query_status()),
            Some(ref pid) => to_status(wrapper_gamemode_query_status_for(pid.get())),
        }
    }

    #[inline]
    pub fn end(self) -> Result<(), Error> {
        let result = self.end_internal();
        mem::forget(self);
        result
    }

    #[inline]
    fn end_internal(&mut self) -> Result<(), Error> {
        let result = match *self.active_pid {
            None => wrapper_gamemode_request_end(),
            Some(ref pid) => wrapper_gamemode_request_end_for(pid.get()),
        };

        if result != 0 {
            Err(error())
        } else {
            Ok(())
        }
    }
}

impl Drop for GameMode {
    #[inline]
    fn drop(&mut self) {
        match self.end_internal() {
            Err(e) if thread::panicking() => {
                eprintln!("{}", e);
            }
            result @ _ => result.expect("An error occurred while ending game mode"),
        }
    }
}

impl fmt::Debug for GameMode {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("GameMode { .. }")
    }
}

#[inline]
fn error() -> Error {
    let reason = unsafe {
        CStr::from_ptr(wrapper_gamemode_error_string())
            .to_string_lossy()
            .into_owned()
    };

    Error::OperationFailed(reason)
}
