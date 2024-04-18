use std::{ffi::c_char, mem, ptr};

use bitflags::bitflags;
use slang_sys::{vtable_call, Interface};

use crate::utils::assert_size_and_align;

mod utils;

pub mod sys {
    pub use slang_sys::*;
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct CompileTarget(i32);

impl CompileTarget {
    pub const UNKNOWN: Self = Self(sys::SlangCompileTarget_SLANG_TARGET_UNKNOWN as _);
    pub const NONE: Self = Self(sys::SlangCompileTarget_SLANG_TARGET_NONE as _);
    pub const GLSL: Self = Self(sys::SlangCompileTarget_SLANG_GLSL as _);
    pub const GLSL_VULKAN: Self = Self(sys::SlangCompileTarget_SLANG_GLSL_VULKAN as _);
    pub const VULKAN_ONE_DESC: Self = Self(sys::SlangCompileTarget_SLANG_GLSL_VULKAN_ONE_DESC as _);
    pub const HLSL: Self = Self(sys::SlangCompileTarget_SLANG_HLSL as _);
    pub const SPIRV: Self = Self(sys::SlangCompileTarget_SLANG_SPIRV as _);
    pub const SPIRV_ASM: Self = Self(sys::SlangCompileTarget_SLANG_SPIRV_ASM as _);
    pub const DXBC: Self = Self(sys::SlangCompileTarget_SLANG_DXBC as _);
    pub const DXBC_ASM: Self = Self(sys::SlangCompileTarget_SLANG_DXBC_ASM as _);
    pub const DXIL: Self = Self(sys::SlangCompileTarget_SLANG_DXIL as _);
    pub const DXIL_ASM: Self = Self(sys::SlangCompileTarget_SLANG_DXIL_ASM as _);
    pub const C_SOURCE: Self = Self(sys::SlangCompileTarget_SLANG_C_SOURCE as _);
    pub const CPP_SOURCE: Self = Self(sys::SlangCompileTarget_SLANG_CPP_SOURCE as _);
    pub const HOST_EXECUTABLE: Self = Self(sys::SlangCompileTarget_SLANG_HOST_EXECUTABLE as _);
    pub const SHADER_SHARED_LIBRARY: Self =
        Self(sys::SlangCompileTarget_SLANG_SHADER_SHARED_LIBRARY as _);
    pub const SHADER_HOST_CALLABLE: Self =
        Self(sys::SlangCompileTarget_SLANG_SHADER_HOST_CALLABLE as _);
    pub const CUDA_SOURCE: Self = Self(sys::SlangCompileTarget_SLANG_CUDA_SOURCE as _);
    pub const PTX: Self = Self(sys::SlangCompileTarget_SLANG_PTX as _);
    pub const CUDA_OBJECT_CODE: Self = Self(sys::SlangCompileTarget_SLANG_CUDA_OBJECT_CODE as _);
    pub const OBJECT_CODE: Self = Self(sys::SlangCompileTarget_SLANG_OBJECT_CODE as _);
    pub const HOST_CPP_SOURCE: Self = Self(sys::SlangCompileTarget_SLANG_HOST_CPP_SOURCE as _);
    pub const HOST_HOST_CALLABLE: Self =
        Self(sys::SlangCompileTarget_SLANG_HOST_HOST_CALLABLE as _);
    pub const CPP_PYTORCH_BINDING: Self =
        Self(sys::SlangCompileTarget_SLANG_CPP_PYTORCH_BINDING as _);
    pub const TARGET_COUNT_OF: Self = Self(sys::SlangCompileTarget_SLANG_TARGET_COUNT_OF as _);
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ProfileID(u32);

impl ProfileID {
    pub const UNKNOWN: Self = Self(sys::SlangProfileID_SLANG_PROFILE_UNKNOWN as _);
}

bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct TargetFlags : u32 {
        const PARAMETER_BLOCKS_USE_REGISTER_SPACES = sys::SLANG_TARGET_FLAG_PARAMETER_BLOCKS_USE_REGISTER_SPACES as _;
        const GENERATE_WHOLE_PROGRAM = sys::SLANG_TARGET_FLAG_GENERATE_WHOLE_PROGRAM as _;
        const DUMP_IR = sys::SLANG_TARGET_FLAG_DUMP_IR as _;
        const GENERATE_SPIRV_DIRECTLY = sys::SLANG_TARGET_FLAG_GENERATE_SPIRV_DIRECTLY as _;
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct FloatingPointMode(u32);

impl FloatingPointMode {
    pub const DEFAULT: Self =
        Self(sys::SlangFloatingPointMode_SLANG_FLOATING_POINT_MODE_DEFAULT as _);
    pub const FAST: Self = Self(sys::SlangFloatingPointMode_SLANG_FLOATING_POINT_MODE_FAST as _);
    pub const PRECISE: Self =
        Self(sys::SlangFloatingPointMode_SLANG_FLOATING_POINT_MODE_PRECISE as _);
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct LineDirectiveMode(u32);

impl LineDirectiveMode {
    pub const DEFAULT: Self =
        Self(sys::SlangLineDirectiveMode_SLANG_LINE_DIRECTIVE_MODE_DEFAULT as _);
    pub const NONE: Self = Self(sys::SlangLineDirectiveMode_SLANG_LINE_DIRECTIVE_MODE_NONE as _);
    pub const STANDARD: Self =
        Self(sys::SlangLineDirectiveMode_SLANG_LINE_DIRECTIVE_MODE_STANDARD as _);
    pub const GLSL: Self = Self(sys::SlangLineDirectiveMode_SLANG_LINE_DIRECTIVE_MODE_GLSL as _);
    pub const SOURCE_MAP: Self =
        Self(sys::SlangLineDirectiveMode_SLANG_LINE_DIRECTIVE_MODE_SOURCE_MAP as _);
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TargetDesc {
    pub structure_size: usize,
    pub format: CompileTarget,
    pub profile: ProfileID,
    pub flags: TargetFlags,
    pub floating_point_mode: FloatingPointMode,
    pub line_directive_mode: LineDirectiveMode,
    pub force_glsl_scalar_buffer_layout: bool,
    pub compiler_option_entries: *mut CompilerOptionEntry,
    pub compiler_option_entry_count: u32,
}

impl Default for TargetDesc {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

assert_size_and_align!(TargetDesc, sys::slang_TargetDesc);

bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct SessionFlags : u32 {
        const NONE = sys::kSessionFlags_None as _;
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct MatrixLayoutMode(u32);

impl MatrixLayoutMode {
    pub const UNKNOWN: Self =
        Self(sys::SlangMatrixLayoutMode_SLANG_MATRIX_LAYOUT_MODE_UNKNOWN as _);
    pub const ROW_MAJOR: Self = Self(sys::SlangMatrixLayoutMode_SLANG_MATRIX_LAYOUT_ROW_MAJOR as _);
    pub const COLUMN_MAJOR: Self =
        Self(sys::SlangMatrixLayoutMode_SLANG_MATRIX_LAYOUT_COLUMN_MAJOR as _);
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PreprocessorMacroDesc {
    pub name: *const c_char,
    pub value: *const c_char,
}

impl Default for PreprocessorMacroDesc {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

assert_size_and_align!(PreprocessorMacroDesc, sys::slang_PreprocessorMacroDesc);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct CompilerOptionName(i32);

impl CompilerOptionName {
    pub const INT: Self = Self(sys::slang_CompilerOptionValueKind_Int as _);
    pub const STRING: Self = Self(sys::slang_CompilerOptionValueKind_String as _);
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CompilerOptionEntry {
    pub name: CompilerOptionName,
    pub value: CompilerOptionName,
}

impl Default for CompilerOptionEntry {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

assert_size_and_align!(CompilerOptionEntry, sys::slang_CompilerOptionEntry);

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SessionDesc {
    pub structure_size: usize,
    pub targets: *const TargetDesc,
    pub target_count: i64,
    pub flags: SessionFlags,
    pub default_matrix_layout_mode: MatrixLayoutMode,
    pub search_paths: *const *const c_char,
    pub search_path_count: i64,
    pub preprocessor_macros: *const PreprocessorMacroDesc,
    pub preprocessor_macro_count: i64,
    pub file_system: *mut sys::ISlangFileSystem,
    pub enable_effect_annotations: bool,
    pub allow_glsl_syntax: bool,
    pub compiler_option_entries: *mut CompilerOptionEntry,
    pub compiler_option_entry_count: u32,
}

impl Default for SessionDesc {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

assert_size_and_align!(SessionDesc, sys::slang_SessionDesc);

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

    pub fn create_session(&self, desc: &SessionDesc) -> utils::Result<Session> {
        let mut session = ptr::null_mut();
        utils::result_from_ffi(unsafe {
            vtable_call!(
                self.0,
                createSession(desc as *const SessionDesc as *const _, &mut session)
            )
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
