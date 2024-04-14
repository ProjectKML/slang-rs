use std::ptr;

use slang_sys::{vtable_call, Interface};

mod utils;

pub mod sys {
    pub use slang_sys::*;
}

pub struct GlobalSession(sys::IGlobalSession);

impl GlobalSession {
    pub fn new() -> utils::Result<Self> {
        let mut session = ptr::null_mut();
        utils::result_from_ffi(unsafe {
            sys::slang_createGlobalSession(sys::SLANG_API_VERSION as _, &mut session)
        })?;

        Ok(Self(unsafe { sys::IGlobalSession::from_raw(session) }))
    }

    pub fn new_without_stdlib() -> utils::Result<Self> {
        let mut session = ptr::null_mut();
        utils::result_from_ffi(unsafe {
            sys::slang_createGlobalSessionWithoutStdLib(sys::SLANG_API_VERSION as _, &mut session)
        })?;

        Ok(Self(unsafe { sys::IGlobalSession::from_raw(session) }))
    }

    pub fn create_session(&self) -> utils::Result<Session> {
        let mut session = ptr::null_mut();
        utils::result_from_ffi(unsafe {
            vtable_call!(self.0, createSession(ptr::null(), &mut session))
        })?;
        Ok(Session(unsafe { sys::ISession::from_raw(session) }))
    }
}

impl Drop for GlobalSession {
    fn drop(&mut self) {
        unsafe {}
    }
}

pub struct Session(sys::ISession);
