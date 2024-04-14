use std::mem;
use crate::{IUnknown, SlangUUID};

pub unsafe trait Interface : Sized {
    const UUID: SlangUUID;

    type Raw;
    type VTable;

    #[inline]
    unsafe fn as_raw(&self) -> *mut Self::Raw {
        mem::transmute_copy(self)
    }

    unsafe fn from_raw(raw: *mut Self::Raw) -> Self;

    #[inline]
    unsafe fn vtable(&self) -> &Self::VTable {
        &**(self.as_raw() as *mut *mut Self::VTable)
    }

    #[inline]
    fn as_unknown(&self) -> &IUnknown {
        unsafe {
            mem::transmute(self)
        }
    }
}

pub use paste::paste;

macro_rules! interface {
    ($name: ident, $sys_name: ident, [$data1: literal, $data2: literal, $data3: literal, [$data41: literal, $data42: literal, $data43: literal, $data44: literal, $data45: literal, $data46: literal, $data47: literal, $data48: literal]], {
        $($fn_name: ident: $fn_ty: ty,)*
    }) => {
        $crate::paste! {
            #[repr(transparent)]
            pub struct $name(*mut $sys_name);

            unsafe impl $crate::Interface for $name {
                const UUID: SlangUUID = SlangUUID { data1: $data1, data2: $data2, data3: $data3, data4: [$data41, $data42, $data43, $data44, $data45, $data46, $data47, $data48] };

                type Raw = $sys_name;
                type VTable = [<$name Vtbl>];

                #[inline]
                unsafe fn from_raw(raw: *mut Self::Raw) -> Self {
                    Self(raw)
                }
            }

            #[repr(C)]
            pub struct [<$name Vtbl>] {
                pub _base: ISlangUnknown__bindgen_vtable,

                $(pub $fn_name: $fn_ty,)*
            }
        }
    };
    ($name: ident, [$data1: literal, $data2: literal, $data3: literal, [$data41: literal, $data42: literal, $data43: literal, $data44: literal, $data45: literal, $data46: literal, $data47: literal, $data48: literal]], {
        $($fn_name: ident: $fn_ty: ty,)*
    }) => {
        $crate::paste! {
            #[repr(transparent)]
            pub struct $name(*mut [<slang_ $name>]);

            unsafe impl $crate::Interface for $name {
                const UUID: SlangUUID = SlangUUID { data1: $data1, data2: $data2, data3: $data3, data4: [$data41, $data42, $data43, $data44, $data45, $data46, $data47, $data48] };

                type Raw = [<slang_ $name>];
                type VTable = [<$name Vtbl>];

                #[inline]
                unsafe fn from_raw(raw: *mut Self::Raw) -> Self {
                    Self(raw)
                }
            }

            #[repr(C)]
            pub struct [<$name Vtbl>] {
                pub _base: ISlangUnknown__bindgen_vtable,

                $(pub $fn_name: $fn_ty,)*
            }
        }
    };
}

pub(crate) use interface;

#[macro_export]
macro_rules! vtable_call {
	($self: expr, $method: ident($($args: expr),*)) => {
		($self.vtable().$method)($self.as_raw().cast(), $($args),*)
	};
}

#[macro_export]
macro_rules! vtable_unknown_call {
    ($self: expr, $method: ident($($args: expr),*)) => {
        ($self.as_unknown().vtable().$method)($self.as_raw().cast(), $($args),*)
    }
}