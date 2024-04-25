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
    pub const MACRO_DEFINE: Self = Self(sys::slang_CompilerOptionName_MacroDefine as _);
    pub const DEP_FILE: Self = Self(sys::slang_CompilerOptionName_DepFile as _);
    pub const ENTRY_POINT_NAME: Self = Self(sys::slang_CompilerOptionName_EntryPointName as _);
    pub const SPECIALIZE: Self = Self(sys::slang_CompilerOptionName_Specialize as _);
    pub const HELP: Self = Self(sys::slang_CompilerOptionName_Help as _);
    pub const HELP_STYLE: Self = Self(sys::slang_CompilerOptionName_HelpStyle as _);
    pub const INCLUDE: Self = Self(sys::slang_CompilerOptionName_Include as _);
    pub const LANGUAGE: Self = Self(sys::slang_CompilerOptionName_Language as _);
    pub const MATRIX_LAYOUT_COLUMN: Self =
        Self(sys::slang_CompilerOptionName_MatrixLayoutColumn as _);
    pub const MATRIX_LAYOUT_ROW: Self = Self(sys::slang_CompilerOptionName_MatrixLayoutRow as _);
    pub const MODULE_NAME: Self = Self(sys::slang_CompilerOptionName_ModuleName as _);
    pub const OUTPUT: Self = Self(sys::slang_CompilerOptionName_Output as _);
    pub const PROFILE: Self = Self(sys::slang_CompilerOptionName_Profile as _);
    pub const STAGE: Self = Self(sys::slang_CompilerOptionName_Stage as _);
    pub const TARGET: Self = Self(sys::slang_CompilerOptionName_Target as _);
    pub const VERSION: Self = Self(sys::slang_CompilerOptionName_Version as _);
    pub const WARNINGS_AS_ERRORS: Self = Self(sys::slang_CompilerOptionName_WarningsAsErrors as _);
    pub const DISABLE_WARNINGS: Self = Self(sys::slang_CompilerOptionName_DisableWarnings as _);
    pub const ENABLE_WARNING: Self = Self(sys::slang_CompilerOptionName_EnableWarning as _);
    pub const DISABLE_WARNING: Self = Self(sys::slang_CompilerOptionName_DisableWarning as _);
    pub const DUMP_WARNING_DIAGNOSTICS: Self =
        Self(sys::slang_CompilerOptionName_DumpWarningDiagnostics as _);
    pub const INPUT_FILES_REMAIN: Self = Self(sys::slang_CompilerOptionName_InputFilesRemain as _);
    pub const EMIT_IR: Self = Self(sys::slang_CompilerOptionName_EmitIr as _);
    pub const REPORT_DOWNSTREAM_TIME: Self =
        Self(sys::slang_CompilerOptionName_ReportDownstreamTime as _);
    pub const REPORT_PERF_BENCHMARK: Self =
        Self(sys::slang_CompilerOptionName_ReportPerfBenchmark as _);
    pub const SKIP_SPIRV_VALIDATION: Self =
        Self(sys::slang_CompilerOptionName_SkipSPIRVValidation as _);
    pub const SOURCE_EMBED_STYLE: Self = Self(sys::slang_CompilerOptionName_SourceEmbedStyle as _);
    pub const SOURCE_EMBED_NAME: Self = Self(sys::slang_CompilerOptionName_SourceEmbedName as _);
    pub const SOURCE_EMBED_LANGUAGE: Self =
        Self(sys::slang_CompilerOptionName_SourceEmbedLanguage as _);
    pub const CAPABILITY: Self = Self(sys::slang_CompilerOptionName_Capability as _);
    pub const DEFAULT_IMAGE_FORMAT_UNKNOWN: Self =
        Self(sys::slang_CompilerOptionName_DefaultImageFormatUnknown as _);
    pub const DISABLE_DYNAMIC_DISPATCH: Self =
        Self(sys::slang_CompilerOptionName_DisableDynamicDispatch as _);
    pub const DISABLE_SPECIALIZATION: Self =
        Self(sys::slang_CompilerOptionName_DisableSpecialization as _);
    pub const FLOATING_POINT_MODE: Self =
        Self(sys::slang_CompilerOptionName_FloatingPointMode as _);
    pub const DEBUG_INFORMATION: Self = Self(sys::slang_CompilerOptionName_DebugInformation as _);
    pub const LINE_DIRECTIVE_MODE: Self =
        Self(sys::slang_CompilerOptionName_LineDirectiveMode as _);
    pub const OPTIMIZATION: Self = Self(sys::slang_CompilerOptionName_Optimization as _);
    pub const OBFUSCATE: Self = Self(sys::slang_CompilerOptionName_Obfuscate as _);
    pub const VULKAN_BIND_SHIFT: Self = Self(sys::slang_CompilerOptionName_VulkanBindShift as _);
    pub const VULKAN_BIND_GLOBALS: Self =
        Self(sys::slang_CompilerOptionName_VulkanBindGlobals as _);
    pub const VULKAN_INVERT_Y: Self = Self(sys::slang_CompilerOptionName_VulkanInvertY as _);
    pub const VULKAN_USE_DX_POSITION_W: Self =
        Self(sys::slang_CompilerOptionName_VulkanUseDxPositionW as _);
    pub const VULKAN_USE_ENTRY_POINT_NAME: Self =
        Self(sys::slang_CompilerOptionName_VulkanUseEntryPointName as _);
    pub const VULKAN_USE_GL_LAYOUT: Self =
        Self(sys::slang_CompilerOptionName_VulkanUseGLLayout as _);
    pub const VULKAN_EMIT_REFLECTION: Self =
        Self(sys::slang_CompilerOptionName_VulkanEmitReflection as _);
    pub const GLSL_FORCE_SCALAR_LAYOUT: Self =
        Self(sys::slang_CompilerOptionName_GLSLForceScalarLayout as _);
    pub const ENABLE_EFFECT_ANNOTATIONS: Self =
        Self(sys::slang_CompilerOptionName_EnableEffectAnnotations as _);
    pub const EMIT_SPIRV_VIA_GLSL: Self = Self(sys::slang_CompilerOptionName_EmitSpirvViaGLSL as _);
    pub const EMIT_SPIRV_DIRECTLY: Self =
        Self(sys::slang_CompilerOptionName_EmitSpirvDirectly as _);
    pub const SPIRV_CORE_GRAMMAR_JSON: Self =
        Self(sys::slang_CompilerOptionName_SPIRVCoreGrammarJSON as _);
    pub const INCOMPLETE_LIBRARY: Self = Self(sys::slang_CompilerOptionName_IncompleteLibrary as _);
    pub const COMPILER_PATH: Self = Self(sys::slang_CompilerOptionName_CompilerPath as _);
    pub const DEFAULT_DOWNSTREAM_COMPILER: Self =
        Self(sys::slang_CompilerOptionName_DefaultDownstreamCompiler as _);
    pub const DOWNSTREAM_ARGS: Self = Self(sys::slang_CompilerOptionName_DownstreamArgs as _);
    pub const PASS_THROUGH: Self = Self(sys::slang_CompilerOptionName_PassThrough as _);
    pub const DUMP_REPRO: Self = Self(sys::slang_CompilerOptionName_DumpRepro as _);
    pub const DUMP_REPRO_ON_ERROR: Self = Self(sys::slang_CompilerOptionName_DumpReproOnError as _);
    pub const EXTRACT_REPRO: Self = Self(sys::slang_CompilerOptionName_ExtractRepro as _);
    pub const LOAD_REPRO: Self = Self(sys::slang_CompilerOptionName_LoadRepro as _);
    pub const LOAD_REPRO_DIRECTORY: Self =
        Self(sys::slang_CompilerOptionName_LoadReproDirectory as _);
    pub const REPRO_FALLBACK_DIRECTORY: Self =
        Self(sys::slang_CompilerOptionName_ReproFallbackDirectory as _);
    pub const DUMP_AST: Self = Self(sys::slang_CompilerOptionName_DumpAst as _);
    pub const DUMP_INTERMEDIATE_PREFIX: Self =
        Self(sys::slang_CompilerOptionName_DumpIntermediatePrefix as _);
    pub const DUMP_INTERMEDIATES: Self = Self(sys::slang_CompilerOptionName_DumpIntermediates as _);
    pub const DUMP_IR: Self = Self(sys::slang_CompilerOptionName_DumpIr as _);
    pub const DUMP_IR_IDS: Self = Self(sys::slang_CompilerOptionName_DumpIrIds as _);
    pub const PREPROCESSOR_OUTPUT: Self =
        Self(sys::slang_CompilerOptionName_PreprocessorOutput as _);
    pub const OUTPUT_INCLUDES: Self = Self(sys::slang_CompilerOptionName_OutputIncludes as _);
    pub const REPRO_FILE_SYSTEM: Self = Self(sys::slang_CompilerOptionName_ReproFileSystem as _);
    pub const SERIAL_IR: Self = Self(sys::slang_CompilerOptionName_SerialIr as _);
    pub const SKIP_CODE_GEN: Self = Self(sys::slang_CompilerOptionName_SkipCodeGen as _);
    pub const VALIDATE_IR: Self = Self(sys::slang_CompilerOptionName_ValidateIr as _);
    pub const VERBOSE_PATHS: Self = Self(sys::slang_CompilerOptionName_VerbosePaths as _);
    pub const VERIFY_DEBUG_SERIAL_IR: Self =
        Self(sys::slang_CompilerOptionName_VerifyDebugSerialIr as _);
    pub const NO_CODE_GEN: Self = Self(sys::slang_CompilerOptionName_NoCodeGen as _);
    pub const FILE_SYSTEM: Self = Self(sys::slang_CompilerOptionName_FileSystem as _);
    pub const HETEROGENEOUS: Self = Self(sys::slang_CompilerOptionName_Heterogeneous as _);
    pub const NO_MANGLE: Self = Self(sys::slang_CompilerOptionName_NoMangle as _);
    pub const NO_HLSL_BINDING: Self = Self(sys::slang_CompilerOptionName_NoHLSLBinding as _);
    pub const VALIDATE_UNIFORMITY: Self =
        Self(sys::slang_CompilerOptionName_ValidateUniformity as _);
    pub const ALLOW_GLSL: Self = Self(sys::slang_CompilerOptionName_AllowGLSL as _);
    pub const ARCHIVE_TYPE: Self = Self(sys::slang_CompilerOptionName_ArchiveType as _);
    pub const COMPILE_STD_LIB: Self = Self(sys::slang_CompilerOptionName_CompileStdLib as _);
    pub const DOC: Self = Self(sys::slang_CompilerOptionName_Doc as _);
    pub const IR_COMPRESSION: Self = Self(sys::slang_CompilerOptionName_IrCompression as _);
    pub const LOAD_STD_LIB: Self = Self(sys::slang_CompilerOptionName_LoadStdLib as _);
    pub const REFERENCE_MODULE: Self = Self(sys::slang_CompilerOptionName_ReferenceModule as _);
    pub const SAVE_STD_LIB: Self = Self(sys::slang_CompilerOptionName_SaveStdLib as _);
    pub const SAVE_STD_LIB_BIN_SOURCE: Self =
        Self(sys::slang_CompilerOptionName_SaveStdLibBinSource as _);
    pub const TRACK_LIVENESS: Self = Self(sys::slang_CompilerOptionName_TrackLiveness as _);
    pub const PARAMETER_BLOCKS_USE_REGISTER_SPACES: Self =
        Self(sys::slang_CompilerOptionName_ParameterBlocksUseRegisterSpaces as _);
    pub const COUNT_OF_PARSABLE_OPTIONS: Self =
        Self(sys::slang_CompilerOptionName_CountOfParsableOptions as _);
    pub const DEBUG_INFORMATION_FORMAT: Self =
        Self(sys::slang_CompilerOptionName_DebugInformationFormat as _);
    pub const VULKAN_BIND_SHIFT_ALL: Self =
        Self(sys::slang_CompilerOptionName_VulkanBindShiftAll as _);
    pub const GENERATE_WHOLE_PROGRAM: Self =
        Self(sys::slang_CompilerOptionName_GenerateWholeProgram as _);
    pub const USE_UP_TO_DATE_BINARY_MODULE: Self =
        Self(sys::slang_CompilerOptionName_UseUpToDateBinaryModule as _);
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct CompilerOptionValueKind(i32);

impl CompilerOptionValueKind {
    pub const INT: Self = Self(sys::slang_CompilerOptionValueKind_Int as _);
    pub const STRING: Self = Self(sys::slang_CompilerOptionValueKind_String as _);
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CompilerOptionValue {
    pub kind: CompilerOptionValueKind,
    pub int_value0: i32,
    pub int_value1: i32,
    pub string_value0: *const c_char,
    pub string_value1: *const c_char,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CompilerOptionEntry {
    pub name: CompilerOptionName,
    pub value: CompilerOptionValue,
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

    #[inline]
    pub unsafe fn create_session(&self, desc: &SessionDesc) -> utils::Result<Session> {
        let mut session = ptr::null_mut();
        utils::result_from_ffi(vtable_call!(
            self.0,
            createSession(desc as *const SessionDesc as *const _, &mut session)
        ))?;
        Ok(Session(sys::ISession::from_raw(session)))
    }

    #[inline]
    pub unsafe fn find_profile(&self, name: *const c_char) -> ProfileID {
        ProfileID(unsafe { vtable_call!(self.0, findProfile(name)) })
    }

    #[inline]
    pub unsafe fn create_compile_request(&self) -> utils::Result<CompileRequest> {
        let mut compile_request = ptr::null_mut();
        utils::result_from_ffi(vtable_call!(
            self.0,
            createCompileRequest(&mut compile_request)
        ))?;
        Ok(CompileRequest(sys::ICompileRequest::from_raw(
            compile_request,
        )))
    }
}

impl Drop for GlobalSession {
    fn drop(&mut self) {
        unsafe {
            //TODO:
        }
    }
}

pub struct Session(sys::ISession);

impl Session {}

pub struct CompileRequest(sys::ICompileRequest);

impl CompileRequest {}
