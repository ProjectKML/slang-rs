use std::mem;
use crate::SlangUUID;

pub unsafe trait Interface : Sized {
    const UUID: SlangUUID;

    type Raw;
    type VTable;

    unsafe fn as_raw(&self) -> *mut Self::Raw {
        mem::transmute_copy(self)
    }

    #[inline]
    unsafe fn vtable(&self) -> &Self::VTable {
        &**(self.as_raw() as *mut *mut Self::VTable)
    }

}

pub use paste::paste;

#[macro_export]
macro_rules! interface {
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
            }

            #[repr(C)]
            pub struct [<$name Vtbl>] {
                pub _base: ISlangUnknown__bindgen_vtable,

                $(pub $fn_name: $fn_ty,)*
            }
        }
    };
}
