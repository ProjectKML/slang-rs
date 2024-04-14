use crate::sys;

pub type Result<T> = std::result::Result<T, sys::SlangResult>;

#[inline]
pub(crate) fn result_from_ffi(result: sys::SlangResult) -> Result<()> {
    if result < 0 {
        Err(result)
    } else {
        Ok(())
    }
}
