mod utils;

pub mod sys {
    pub use slang_sys::*;
}

pub struct GlobalSession(*mut sys::slang_IGlobalSession);
