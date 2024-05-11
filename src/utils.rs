use crate::{sys, Blob};

pub type Result<T> = std::result::Result<T, sys::SlangResult>;

#[inline]
pub(crate) fn result_from_ffi(result: sys::SlangResult) -> Result<()> {
    if result < 0 {
        Err(result)
    } else {
        Ok(())
    }
}

macro_rules! define_interface {
    ($name: ident, $sys_ty: ty) => {
        paste::paste! {
            #[repr(transparent)]
            #[derive(Debug)]
            pub struct $name(*mut $sys_ty);

            impl $name {
                #[inline]
                pub fn as_raw(&self) -> *mut $sys_ty {
                    self.0
                }
            }

            impl Clone for $name {
                fn clone(&self) -> Self {
                    unsafe {
                        ((*self.0).unknown_vtable().ISlangUnknown_addRef)(self.0.cast());
                    }
                    Self(self.0.cast())
                }
            }
        }
    };

    ($name: ident, $sys_ty: ty, $base_ty: ty) => {
        paste::paste! {
            #[repr(transparent)]
            #[derive(Debug)]
            pub struct $name(*mut $sys_ty);

            impl $name {
                #[inline]
                pub fn as_raw(&self) -> *mut $sys_ty {
                    self.0
                }
            }

            impl Deref for $name {
                type Target = $base_ty;

                #[inline]
                fn deref(&self) -> &Self::Target {
                    unsafe { mem::transmute(self) }
                }
            }
        }
    };
}

pub(crate) use define_interface;

macro_rules! assert_size_and_align {
    ($type_: ty, $ffi_type: ty) => {
        ::static_assertions::assert_eq_size!($type_, $ffi_type);
        ::static_assertions::assert_eq_align!($type_, $ffi_type);
    };
}

pub(crate) use assert_size_and_align;

pub(crate) const S_OK: sys::SlangResult = 0;
pub(crate) const E_INVALIDARG: sys::SlangResult = -2147024809;
pub(crate) const E_NOINTERFACE: sys::SlangResult = -2147467262;
