use std::{
    borrow::Cow,
    ffi::{c_char, c_void, CStr, CString},
    mem, ptr, slice,
};

use bitflags::bitflags;
use slang_sys::{vtable_call, Interface, SlangReflection};

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
    pub fn find_profile(&self, name: &str) -> ProfileID {
        let name = CString::new(name).unwrap();
        ProfileID(unsafe { vtable_call!(self.0, findProfile(name.as_ptr())) })
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

pub struct FileSystem(sys::ISlangFileSystem);

pub struct Session(sys::ISession);

impl Session {
    #[inline]
    pub fn get_global_session(&mut self) -> GlobalSession {
        let mut global_session = unsafe { vtable_call!(self.0, getGlobalSession()) };
        GlobalSession(unsafe { sys::IGlobalSession::from_raw(global_session) })
    }

    #[inline]
    pub fn load_module(&mut self, module_name: &str) -> utils::Result<(Module, Blob)> {
        let mut diagnostics = ptr::null_mut();

        let module_name = CString::new(module_name).unwrap();

        let module = unsafe { vtable_call!(self.0, loadModule(module_name.as_ptr(), &mut diagnostics)) };

        if module.is_null() {
            utils::Result::Err(-1)
        } else {
            Ok((
                Module(unsafe { sys::IModule::from_raw(module) }),
                Blob(unsafe { sys::IBlob::from_raw(diagnostics) }),
            ))
        }
    }

    #[inline]
    pub fn load_module_from_source(
        &mut self,
        module_name: &str,
        path: &str,
        source: &Blob,
    ) -> utils::Result<(Module, Blob)> {
        let mut diagnostics = ptr::null_mut();

        let module_name = CString::new(module_name).unwrap();
        let path = CString::new(path).unwrap();

        let module = unsafe { vtable_call!(
            self.0,
            loadModuleFromSource(
                module_name.as_ptr(),
                path.as_ptr(),
                source.0.as_raw(),
                &mut diagnostics
            )
        )};

        if module.is_null() {
            utils::Result::Err(-1)
        } else {
            Ok((
                Module(unsafe { sys::IModule::from_raw(module) }),
                Blob(unsafe { sys::IBlob::from_raw(diagnostics) }),
            ))
        }
    }

    #[inline]
    pub unsafe fn create_composite_component_type(
        &mut self,
        component_types: &[ComponentType],
    ) -> utils::Result<(ComponentType, Blob)> {
        let mut composite_component_type = ptr::null_mut();
        let mut diagnostics = ptr::null_mut();

        utils::result_from_ffi(vtable_call!(
            self.0,
            createCompositeComponentType(
                component_types.as_ptr().cast(),
                component_types.len() as _,
                &mut composite_component_type,
                &mut diagnostics
            )
        ))?;
        Ok((
            ComponentType(sys::IComponentType::from_raw(composite_component_type)),
            Blob(sys::IBlob::from_raw(diagnostics)),
        ))
    }

    //TODO: specializeType

    //TODO: getTypeLayout

    //TODO: getContainerType

    //TODO: getDynamicType

    //TODO: getTypeRTTIMangledName

    //TODO: getTypeConformanceWitnessMangledName

    //TODO: getTypeConformanceWitnessSequentialID

    #[inline]
    pub unsafe fn create_compile_request(&mut self) -> utils::Result<CompileRequest> {
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

pub struct Writer(sys::IWriter);

pub struct SharedLibrary(sys::ISharedLibrary);

pub struct MutableFileSystem(sys::IMutableFileSystem);

pub struct ComponentType(sys::IComponentType);

pub struct EntryPoint(sys::IEntryPoint);

pub struct Module(sys::IModule);

impl Module {
    #[inline]
    pub unsafe fn find_entry_point_by_name(&mut self, name: &str) -> utils::Result<EntryPoint> {
        let name = CString::new(name).unwrap();
        let mut entry_point = ptr::null_mut();
        utils::result_from_ffi(vtable_call!(
            self.0,
            findEntryPointByName(name.as_ptr(), &mut entry_point)
        ))?;
        Ok(EntryPoint(sys::IEntryPoint::from_raw(entry_point)))
    }

    #[inline]
    pub unsafe fn get_defined_entry_point_count(&mut self) -> i32 {
        vtable_call!(self.0, getDefinedEntryPointCount())
    }

    #[inline]
    pub unsafe fn get_defined_entry_point(&mut self, index: i32) -> utils::Result<EntryPoint> {
        let mut entry_point = ptr::null_mut();
        utils::result_from_ffi(vtable_call!(
            self.0,
            getDefinedEntryPoint(index, &mut entry_point)
        ))?;
        Ok(EntryPoint(sys::IEntryPoint::from_raw(entry_point)))
    }

    #[inline]
    pub unsafe fn serialize(&mut self) -> utils::Result<Blob> {
        let mut blob = ptr::null_mut();
        utils::result_from_ffi(vtable_call!(self.0, serialize(&mut blob)))?;
        Ok(Blob(sys::IBlob::from_raw(blob)))
    }

    #[inline]
    pub unsafe fn write_to_file(&mut self, file_name: &str) -> utils::Result<()> {
        let file_name = CString::new(file_name).unwrap();
        utils::result_from_ffi(vtable_call!(self.0, writeToFile(file_name.as_ptr())))
    }

    #[inline]
    pub unsafe fn get_name(&mut self) -> Cow<'_, str> {
        let c_str = CStr::from_ptr(vtable_call!(self.0, getName()));
        c_str.to_string_lossy()
    }

    #[inline]
    pub unsafe fn get_file_path(&mut self) -> Cow<'_, str> {
        let c_str = CStr::from_ptr(vtable_call!(self.0, getFilePath()));
        c_str.to_string_lossy()
    }

    #[inline]
    pub unsafe fn get_unique_identity(&mut self) -> Cow<'_, str> {
        let c_str = CStr::from_ptr(vtable_call!(self.0, getUniqueIdentity()));
        c_str.to_string_lossy()
    }

    #[inline]
    pub unsafe fn find_and_check_entry_point(
        &mut self,
        name: &str,
        stage: Stage,
    ) -> utils::Result<(EntryPoint, Blob)> {
        let name = CString::new(name).unwrap();
        let mut entry_point = ptr::null_mut();
        let mut diagnostics = ptr::null_mut();
        utils::result_from_ffi(vtable_call!(
            self.0,
            findAndCheckEntryPoint(name.as_ptr(), stage.0, &mut entry_point, &mut diagnostics)
        ))?;
        Ok((
            EntryPoint(sys::IEntryPoint::from_raw(entry_point)),
            Blob(sys::IBlob::from_raw(diagnostics)),
        ))
    }
}

pub struct Blob(sys::IBlob);

impl Blob {
    #[inline]
    pub unsafe fn get_buffer_pointer(&self, buffer: *mut c_void) -> *const c_void {
        vtable_call!(self.0, getBufferPointer())
    }

    #[inline]
    pub unsafe fn get_buffer_size(&self) -> usize {
        vtable_call!(self.0, getBufferSize())
    }
}

impl Session {}

bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct CompilerFlags : u32 {
        const NO_MANGLING = sys::SLANG_COMPILE_FLAG_NO_MANGLING as _;
        const NO_CODEGEN = sys::SLANG_COMPILE_FLAG_NO_CODEGEN as _;
        const OBFUSCATE = sys::SLANG_COMPILE_FLAG_OBFUSCATE as _;
        const NO_CHECKING = sys::SLANG_COMPILE_FLAG_NO_CHECKING as _;
        const SPLIT_MIXED_TYPES = sys::SLANG_COMPILE_FLAG_SPLIT_MIXED_TYPES as _;
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct DebugInfoLevel(u32);

impl DebugInfoLevel {
    pub const NONE: Self = Self(sys::SlangDebugInfoLevel_SLANG_DEBUG_INFO_LEVEL_NONE as _);
    pub const MINIMAL: Self = Self(sys::SlangDebugInfoLevel_SLANG_DEBUG_INFO_LEVEL_MINIMAL as _);
    pub const STANDARD: Self = Self(sys::SlangDebugInfoLevel_SLANG_DEBUG_INFO_LEVEL_STANDARD as _);
    pub const MAXIMAL: Self = Self(sys::SlangDebugInfoLevel_SLANG_DEBUG_INFO_LEVEL_MAXIMAL as _);
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct OptimizationLevel(u32);

impl OptimizationLevel {
    pub const NONE: Self = Self(sys::SlangOptimizationLevel_SLANG_OPTIMIZATION_LEVEL_NONE as _);
    pub const DEFAULT: Self =
        Self(sys::SlangOptimizationLevel_SLANG_OPTIMIZATION_LEVEL_DEFAULT as _);
    pub const HIGH: Self = Self(sys::SlangOptimizationLevel_SLANG_OPTIMIZATION_LEVEL_HIGH as _);
    pub const MAXIMAL: Self =
        Self(sys::SlangOptimizationLevel_SLANG_OPTIMIZATION_LEVEL_MAXIMAL as _);
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ContainerFormat(i32);

impl ContainerFormat {
    pub const NONE: Self = Self(sys::SlangContainerFormat_SLANG_CONTAINER_FORMAT_NONE as _);
    pub const MODULE: Self =
        Self(sys::SlangContainerFormat_SLANG_CONTAINER_FORMAT_SLANG_MODULE as _);
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct PassThrough(i32);

impl PassThrough {
    pub const NONE: Self = Self(sys::SlangPassThrough_SLANG_PASS_THROUGH_NONE as _);
    pub const FXC: Self = Self(sys::SlangPassThrough_SLANG_PASS_THROUGH_FXC as _);
    pub const DXC: Self = Self(sys::SlangPassThrough_SLANG_PASS_THROUGH_DXC as _);
    pub const GLSLANG: Self = Self(sys::SlangPassThrough_SLANG_PASS_THROUGH_GLSLANG as _);
    pub const SPIRV_DIS: Self = Self(sys::SlangPassThrough_SLANG_PASS_THROUGH_SPIRV_DIS as _);
}

pub type DiagnosticCallback =
    Option<unsafe extern "C" fn(message: *const c_char, user_data: *mut c_void)>;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct CapabilityID(i32);
impl CapabilityID {
    pub const UNKNOWN: Self = Self(sys::SlangCapabilityID_SLANG_CAPABILITY_UNKNOWN);
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct WriterChannel(u32);

impl WriterChannel {
    pub const DIAGNOSTIC: Self = Self(sys::SlangWriterChannel_SLANG_WRITER_CHANNEL_DIAGNOSTIC);
    pub const STD_OUTPUT: Self = Self(sys::SlangWriterChannel_SLANG_WRITER_CHANNEL_STD_OUTPUT);
    pub const STD_ERROR: Self = Self(sys::SlangWriterChannel_SLANG_WRITER_CHANNEL_STD_ERROR);
    pub const COUNT_OF: Self = Self(sys::SlangWriterChannel_SLANG_WRITER_CHANNEL_COUNT_OF);
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct SourceLanguage(i32);

impl SourceLanguage {
    pub const UNKNOWN: Self = Self(sys::SlangSourceLanguage_SLANG_SOURCE_LANGUAGE_UNKNOWN);
    pub const SLANG: Self = Self(sys::SlangSourceLanguage_SLANG_SOURCE_LANGUAGE_SLANG);
    pub const HLSL: Self = Self(sys::SlangSourceLanguage_SLANG_SOURCE_LANGUAGE_HLSL);
    pub const GLSL: Self = Self(sys::SlangSourceLanguage_SLANG_SOURCE_LANGUAGE_GLSL);
    pub const C: Self = Self(sys::SlangSourceLanguage_SLANG_SOURCE_LANGUAGE_C);
    pub const CPP: Self = Self(sys::SlangSourceLanguage_SLANG_SOURCE_LANGUAGE_CPP);
    pub const CUDA: Self = Self(sys::SlangSourceLanguage_SLANG_SOURCE_LANGUAGE_CUDA);
    pub const SPIRV: Self = Self(sys::SlangSourceLanguage_SLANG_SOURCE_LANGUAGE_SPIRV);
    pub const COUNT_OF: Self = Self(sys::SlangSourceLanguage_SLANG_SOURCE_LANGUAGE_COUNT_OF);
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Stage(u32);

impl Stage {
    pub const NONE: Self = Self(sys::SlangStage_SLANG_STAGE_NONE);
    pub const VERTEX: Self = Self(sys::SlangStage_SLANG_STAGE_VERTEX);
    pub const HULL: Self = Self(sys::SlangStage_SLANG_STAGE_HULL);
    pub const DOMAIN: Self = Self(sys::SlangStage_SLANG_STAGE_DOMAIN);
    pub const GEOMETRY: Self = Self(sys::SlangStage_SLANG_STAGE_GEOMETRY);
    pub const FRAGMENT: Self = Self(sys::SlangStage_SLANG_STAGE_FRAGMENT);
    pub const COMPUTE: Self = Self(sys::SlangStage_SLANG_STAGE_COMPUTE);
    pub const RAY_GENERATION: Self = Self(sys::SlangStage_SLANG_STAGE_RAY_GENERATION);
    pub const INTERSECTION: Self = Self(sys::SlangStage_SLANG_STAGE_INTERSECTION);
    pub const ANY_HIT: Self = Self(sys::SlangStage_SLANG_STAGE_ANY_HIT);
    pub const CLOSEST_HIT: Self = Self(sys::SlangStage_SLANG_STAGE_CLOSEST_HIT);
    pub const MISS: Self = Self(sys::SlangStage_SLANG_STAGE_MISS);
    pub const CALLABLE: Self = Self(sys::SlangStage_SLANG_STAGE_CALLABLE);
    pub const MESH: Self = Self(sys::SlangStage_SLANG_STAGE_MESH);
    pub const AMPLIFICATION: Self = Self(sys::SlangStage_SLANG_STAGE_AMPLIFICATION);
    pub const PIXEL: Self = Self(sys::SlangStage_SLANG_STAGE_PIXEL);
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ParameterCategory(u32);

impl ParameterCategory {
    pub const NONE: Self = Self(sys::SlangParameterCategory_SLANG_PARAMETER_CATEGORY_NONE);
    pub const MIXED: Self = Self(sys::SlangParameterCategory_SLANG_PARAMETER_CATEGORY_MIXED);
    pub const CONSTANT_BUFFER: Self =
        Self(sys::SlangParameterCategory_SLANG_PARAMETER_CATEGORY_CONSTANT_BUFFER);
    pub const SHADER_RESOURCE: Self =
        Self(sys::SlangParameterCategory_SLANG_PARAMETER_CATEGORY_SHADER_RESOURCE);
    pub const UNORDERED_ACCESS: Self =
        Self(sys::SlangParameterCategory_SLANG_PARAMETER_CATEGORY_UNORDERED_ACCESS);
    pub const VARYING_INPUT: Self =
        Self(sys::SlangParameterCategory_SLANG_PARAMETER_CATEGORY_VARYING_INPUT);
    pub const VARYING_OUTPUT: Self =
        Self(sys::SlangParameterCategory_SLANG_PARAMETER_CATEGORY_VARYING_OUTPUT);
    pub const SAMPLER_STATE: Self =
        Self(sys::SlangParameterCategory_SLANG_PARAMETER_CATEGORY_SAMPLER_STATE);
    pub const UNIFORM: Self = Self(sys::SlangParameterCategory_SLANG_PARAMETER_CATEGORY_UNIFORM);
    pub const DESCRIPTOR_TABLE_SLOT: Self =
        Self(sys::SlangParameterCategory_SLANG_PARAMETER_CATEGORY_DESCRIPTOR_TABLE_SLOT);
    pub const SPECIALIZATION_CONSTANT: Self =
        Self(sys::SlangParameterCategory_SLANG_PARAMETER_CATEGORY_SPECIALIZATION_CONSTANT);
    pub const PUSH_CONSTANT_BUFFER: Self =
        Self(sys::SlangParameterCategory_SLANG_PARAMETER_CATEGORY_PUSH_CONSTANT_BUFFER);
    pub const REGISTER_SPACE: Self =
        Self(sys::SlangParameterCategory_SLANG_PARAMETER_CATEGORY_REGISTER_SPACE);
    pub const GENERIC: Self = Self(sys::SlangParameterCategory_SLANG_PARAMETER_CATEGORY_GENERIC);
    pub const RAY_PAYLOAD: Self =
        Self(sys::SlangParameterCategory_SLANG_PARAMETER_CATEGORY_RAY_PAYLOAD);
    pub const HIT_ATTRIBUTES: Self =
        Self(sys::SlangParameterCategory_SLANG_PARAMETER_CATEGORY_HIT_ATTRIBUTES);
    pub const CALLABLE_PAYLOAD: Self =
        Self(sys::SlangParameterCategory_SLANG_PARAMETER_CATEGORY_CALLABLE_PAYLOAD);
    pub const SHADER_RECORD: Self =
        Self(sys::SlangParameterCategory_SLANG_PARAMETER_CATEGORY_SHADER_RECORD);
    pub const EXISTENTIAL_TYPE_PARAM: Self =
        Self(sys::SlangParameterCategory_SLANG_PARAMETER_CATEGORY_EXISTENTIAL_TYPE_PARAM);
    pub const EXISTENTIAL_OBJECT_PARAM: Self =
        Self(sys::SlangParameterCategory_SLANG_PARAMETER_CATEGORY_EXISTENTIAL_OBJECT_PARAM);
    pub const SUB_ELEMENT_REGISTER_SPACE: Self =
        Self(sys::SlangParameterCategory_SLANG_PARAMETER_CATEGORY_SUB_ELEMENT_REGISTER_SPACE);
    pub const SUBPASS: Self = Self(sys::SlangParameterCategory_SLANG_PARAMETER_CATEGORY_SUBPASS);
    pub const COUNT: Self = Self(sys::SlangParameterCategory_SLANG_PARAMETER_CATEGORY_COUNT);
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Severity(i32);

impl Severity {
    pub const DISABLED: Self = Self(sys::SlangSeverity_SLANG_SEVERITY_DISABLED);
    pub const NOTE: Self = Self(sys::SlangSeverity_SLANG_SEVERITY_NOTE);
    pub const WARNING: Self = Self(sys::SlangSeverity_SLANG_SEVERITY_WARNING);
    pub const ERROR: Self = Self(sys::SlangSeverity_SLANG_SEVERITY_ERROR);
    pub const FATAL: Self = Self(sys::SlangSeverity_SLANG_SEVERITY_FATAL);
    pub const INTERNAL: Self = Self(sys::SlangSeverity_SLANG_SEVERITY_INTERNAL);
}

bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
    pub struct DiagnosticFlags : u32 {
        const VERBOSE_PATHS = 1;
        const TREAT_WARNINGS_AS_ERRORS = 2;
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct DebugInfoFormat(u32);

impl DebugInfoFormat {
    pub const DEFAULT: Self = Self(sys::SlangDebugInfoFormat_SLANG_DEBUG_INFO_FORMAT_DEFAULT);
    pub const C7: Self = Self(sys::SlangDebugInfoFormat_SLANG_DEBUG_INFO_FORMAT_C7);
    pub const PDB: Self = Self(sys::SlangDebugInfoFormat_SLANG_DEBUG_INFO_FORMAT_PDB);
    pub const STABS: Self = Self(sys::SlangDebugInfoFormat_SLANG_DEBUG_INFO_FORMAT_STABS);
    pub const COFF: Self = Self(sys::SlangDebugInfoFormat_SLANG_DEBUG_INFO_FORMAT_COFF);
    pub const DWARF: Self = Self(sys::SlangDebugInfoFormat_SLANG_DEBUG_INFO_FORMAT_DWARF);
    pub const COUNT_OF: Self = Self(sys::SlangDebugInfoFormat_SLANG_DEBUG_INFO_FORMAT_COUNT_OF);
}

pub struct CompileRequest(sys::ICompileRequest);

impl CompileRequest {
    #[inline]
    pub unsafe fn set_file_system(&mut self, file_system: &mut FileSystem) {
        vtable_call!(self.0, setFileSystem(&mut file_system.0))
    }

    #[inline]
    pub fn set_compile_flags(&mut self, flags: CompilerFlags) {
        unsafe { vtable_call!(self.0, setCompileFlags(mem::transmute(flags))) }
    }

    #[inline]
    pub fn get_compile_flags(&self) -> CompilerFlags {
        unsafe { mem::transmute(vtable_call!(self.0, getCompileFlags())) }
    }

    #[inline]
    pub unsafe fn set_dump_intermediates(&mut self, enable: i32) {
        vtable_call!(self.0, setDumpIntermediates(enable))
    }

    #[inline]
    pub unsafe fn set_dump_intermediate_prefix(&mut self, prefix: &str) {
        let prefix = CString::new(prefix).unwrap();
        vtable_call!(self.0, setDumpIntermediatePrefix(prefix.as_ptr()))
    }

    #[inline]
    pub unsafe fn set_line_directive_mode(&mut self, mode: LineDirectiveMode) {
        vtable_call!(self.0, setLineDirectiveMode(mode.0))
    }

    #[inline]
    pub unsafe fn set_code_gen_target(&mut self, target: CompileTarget) {
        vtable_call!(self.0, setCodeGenTarget(target.0))
    }

    #[inline]
    pub unsafe fn add_code_gen_target(&mut self, target: CompileTarget) -> i32 {
        vtable_call!(self.0, addCodeGenTarget(target.0))
    }

    #[inline]
    pub unsafe fn set_target_profile(&mut self, target_index: i32, profile: ProfileID) {
        vtable_call!(self.0, setTargetProfile(target_index, profile.0))
    }

    #[inline]
    pub unsafe fn set_target_flags(&mut self, target_index: i32, flags: TargetFlags) {
        vtable_call!(self.0, setTargetFlags(target_index, mem::transmute(flags)))
    }

    #[inline]
    pub unsafe fn set_target_floating_point_mode(
        &mut self,
        target_index: i32,
        mode: FloatingPointMode,
    ) {
        vtable_call!(self.0, setTargetFloatingPointMode(target_index, mode.0))
    }

    #[inline]
    pub unsafe fn set_target_matrix_layout_mode(
        &mut self,
        target_index: i32,
        mode: MatrixLayoutMode,
    ) {
        vtable_call!(self.0, setTargetMatrixLayoutMode(target_index, mode.0))
    }

    #[inline]
    pub unsafe fn set_matrix_layout_mode(&mut self, mode: MatrixLayoutMode) {
        vtable_call!(self.0, setMatrixLayoutMode(mode.0))
    }

    #[inline]
    pub unsafe fn set_debug_info_level(&mut self, level: DebugInfoLevel) {
        vtable_call!(self.0, setDebugInfoLevel(level.0))
    }

    #[inline]
    pub unsafe fn set_optimization_level(&mut self, level: OptimizationLevel) {
        vtable_call!(self.0, setOptimizationLevel(level.0))
    }

    #[inline]
    pub unsafe fn set_output_container_format(&mut self, format: ContainerFormat) {
        vtable_call!(self.0, setOutputContainerFormat(format.0))
    }

    #[inline]
    pub unsafe fn set_pass_through(&mut self, pass_through: PassThrough) {
        vtable_call!(self.0, setPassThrough(pass_through.0))
    }

    #[inline]
    pub unsafe fn set_diagnostic_callback(
        &mut self,
        callback: DiagnosticCallback,
        user_data: *const c_void,
    ) {
        vtable_call!(
            self.0,
            setDiagnosticCallback(mem::transmute(callback), user_data)
        )
    }

    #[inline]
    pub unsafe fn set_writer(&mut self, channel: WriterChannel, writer: Writer) {
        vtable_call!(self.0, setWriter(channel.0, writer.0.as_raw()))
    }

    #[inline]
    pub unsafe fn get_writer(&mut self, channel: WriterChannel) -> Writer {
        let mut writer = vtable_call!(self.0, getWriter(channel.0));
        Writer(sys::IWriter::from_raw(writer.cast()))
    }

    #[inline]
    pub unsafe fn add_search_path(&mut self, search_dir: &str) {
        let search_dir = CString::new(search_dir).unwrap();

        vtable_call!(self.0, addSearchPath(search_dir.as_ptr()))
    }

    #[inline]
    pub unsafe fn add_preprocessor_define(&mut self, key: &str, value: &str) {
        let key = CString::new(key).unwrap();
        let value = CString::new(value).unwrap();

        vtable_call!(self.0, addPreprocessorDefine(key.as_ptr(), value.as_ptr()))
    }

    #[inline]
    pub unsafe fn process_command_line_arguments(&mut self, args: &[&str]) -> utils::Result<()> {
        let args = args
            .into_iter()
            .map(|arg| CString::new(*arg).unwrap())
            .collect::<Vec<_>>();
        let arg_ptrs = args.iter().map(|arg| arg.as_ptr()).collect::<Vec<_>>();

        utils::result_from_ffi(vtable_call!(
            self.0,
            processCommandLineArguments(arg_ptrs.as_ptr(), arg_ptrs.len() as _)
        ))
    }

    #[inline]
    pub unsafe fn add_translation_unit(&mut self, language: SourceLanguage, name: &str) -> i32 {
        let name = CString::new(name).unwrap();
        vtable_call!(self.0, addTranslationUnit(language.0, name.as_ptr()))
    }

    #[inline]
    pub unsafe fn set_default_module_name(&mut self, default_module_name: &str) {
        let default_module_name = CString::new(default_module_name).unwrap();
        vtable_call!(self.0, setDefaultModuleName(default_module_name.as_ptr()))
    }

    #[inline]
    pub unsafe fn add_translation_unit_preprocessor_define(
        &mut self,
        translation_unit_index: i32,
        key: &str,
        value: &str,
    ) {
        let key = CString::new(key).unwrap();
        let value = CString::new(value).unwrap();
        vtable_call!(
            self.0,
            addTranslationUnitPreprocessorDefine(
                translation_unit_index,
                key.as_ptr(),
                value.as_ptr()
            )
        )
    }

    #[inline]
    pub unsafe fn add_translation_unit_source_file(
        &mut self,
        translation_unit_index: i32,
        path: &str,
    ) {
        let path = CString::new(path).unwrap();
        vtable_call!(
            self.0,
            addTranslationUnitSourceFile(translation_unit_index, path.as_ptr())
        )
    }

    #[inline]
    pub unsafe fn add_translation_unit_source_string(
        &mut self,
        translation_unit_index: i32,
        path: &str,
        source: &str,
    ) {
        let path = CString::new(path).unwrap();
        let source = CString::new(source).unwrap();
        vtable_call!(
            self.0,
            addTranslationUnitSourceString(translation_unit_index, path.as_ptr(), source.as_ptr())
        )
    }

    #[inline]
    pub unsafe fn add_library_reference(
        &mut self,
        base_path: &str,
        lib_data: &[u8],
    ) -> utils::Result<()> {
        let base_path = CString::new(base_path).unwrap();

        utils::result_from_ffi(vtable_call!(
            self.0,
            addLibraryReference(
                base_path.as_ptr(),
                lib_data.as_ptr().cast(),
                lib_data.len() as _
            )
        ))
    }

    #[inline]
    pub unsafe fn add_translation_unit_source_string_span(
        &mut self,
        translation_unit_index: i32,
        path: &str,
        source: &str,
    ) {
        let path = CString::new(path).unwrap();
        let source_c = CString::new(source).unwrap();
        vtable_call!(
            self.0,
            addTranslationUnitSourceStringSpan(
                translation_unit_index,
                path.as_ptr(),
                source_c.as_ptr(),
                source_c.as_ptr().add(source.len())
            )
        )
    }

    #[inline]
    pub unsafe fn add_translation_unit_source_blob(
        &mut self,
        translation_unit_index: i32,
        path: &str,
        source_blob: &mut Blob,
    ) {
        let path = CString::new(path).unwrap();
        vtable_call!(
            self.0,
            addTranslationUnitSourceBlob(
                translation_unit_index,
                path.as_ptr(),
                source_blob.0.as_raw()
            )
        )
    }

    #[inline]
    pub unsafe fn add_entry_point(
        &mut self,
        translation_unit_index: i32,
        name: *const c_char,
        stage: Stage,
    ) -> i32 {
        vtable_call!(self.0, addEntryPoint(translation_unit_index, name, stage.0))
    }

    #[inline]
    pub unsafe fn add_entry_point_ex(
        &mut self,
        translation_unit_index: i32,
        name: &str,
        stage: Stage,
        generic_args: &[&str],
    ) -> i32 {
        let name = CString::new(name).unwrap();
        let args = generic_args
            .into_iter()
            .map(|arg| CString::new(*arg).unwrap())
            .collect::<Vec<_>>();
        let arg_ptrs = args.iter().map(|arg| arg.as_ptr()).collect::<Vec<_>>();

        vtable_call!(
            self.0,
            addEntryPointEx(
                translation_unit_index,
                name.as_ptr(),
                stage.0,
                arg_ptrs.len() as _,
                arg_ptrs.as_ptr()
            )
        )
    }

    #[inline]
    pub unsafe fn set_global_generic_args(&mut self, generic_args: &[&str]) -> utils::Result<()> {
        let args = generic_args
            .into_iter()
            .map(|arg| CString::new(*arg).unwrap())
            .collect::<Vec<_>>();
        let arg_ptrs = args.iter().map(|arg| arg.as_ptr()).collect::<Vec<_>>();

        utils::result_from_ffi(vtable_call!(
            self.0,
            setGlobalGenericArgs(arg_ptrs.len() as _, arg_ptrs.as_ptr())
        ))
    }

    #[inline]
    pub unsafe fn set_type_name_for_global_existential_type_param(
        &mut self,
        slot_index: i32,
        type_name: &str,
    ) -> utils::Result<()> {
        let type_name = CString::new(type_name).unwrap();
        utils::result_from_ffi(vtable_call!(
            self.0,
            setTypeNameForGlobalExistentialTypeParam(slot_index, type_name.as_ptr())
        ))
    }

    #[inline]
    pub unsafe fn set_type_name_for_entry_point_existential_type_param(
        &mut self,
        entry_point_index: i32,
        slot_index: i32,
        type_name: &str,
    ) -> utils::Result<()> {
        let type_name = CString::new(type_name).unwrap();
        utils::result_from_ffi(vtable_call!(
            self.0,
            setTypeNameForEntryPointExistentialTypeParam(
                entry_point_index,
                slot_index,
                type_name.as_ptr()
            )
        ))
    }

    #[inline]
    pub unsafe fn set_allow_glsl_input(&mut self, value: bool) {
        vtable_call!(self.0, setAllowGLSLInput(value))
    }

    #[inline]
    pub unsafe fn compile(&mut self) -> utils::Result<()> {
        utils::result_from_ffi(vtable_call!(self.0, compile()))
    }

    #[inline]
    pub unsafe fn get_diagnostic_output(&mut self) -> Cow<'_, str> {
        let c_str = CStr::from_ptr(vtable_call!(self.0, getDiagnosticOutput()));

        c_str.to_string_lossy()
    }

    #[inline]
    pub unsafe fn get_diagnostic_output_blob(&mut self) -> utils::Result<Blob> {
        let mut blob = ptr::null_mut();
        utils::result_from_ffi(vtable_call!(self.0, getDiagnosticOutputBlob(&mut blob)))?;

        Ok(Blob(sys::IBlob::from_raw(blob)))
    }

    #[inline]
    pub unsafe fn get_dependency_file_count(&mut self) -> i32 {
        vtable_call!(self.0, getDependencyFileCount())
    }

    #[inline]
    pub unsafe fn get_dependency_file_path(&mut self, index: i32) -> utils::Result<Cow<'_, str>> {
        let c_str = vtable_call!(self.0, getDependencyFilePath(index));
        if c_str.is_null() {
            Err(1)
        } else {
            Ok(CStr::from_ptr(c_str).to_string_lossy())
        }
    }

    #[inline]
    pub unsafe fn get_translation_unit_count(&mut self) -> i32 {
        vtable_call!(self.0, getTranslationUnitCount())
    }

    ///

    #[inline]
    pub unsafe fn get_entry_point_source(
        &mut self,
        entry_point_index: i32,
    ) -> utils::Result<Cow<'_, str>> {
        let c_str = vtable_call!(self.0, getEntryPointSource(entry_point_index));
        if c_str.is_null() {
            Err(1)
        } else {
            Ok(CStr::from_ptr(c_str).to_string_lossy())
        }
    }

    #[inline]
    pub unsafe fn get_entry_point_code(&mut self, entry_point_index: i32) -> &[u8] {
        let mut len = 0;
        let data = vtable_call!(self.0, getEntryPointCode(entry_point_index, &mut len));
        slice::from_raw_parts(data.cast(), len)
    }

    #[inline]
    pub unsafe fn get_entry_point_code_blob(
        &mut self,
        entry_point_index: i32,
        target_index: i32,
    ) -> utils::Result<Blob> {
        let mut blob = ptr::null_mut();
        utils::result_from_ffi(vtable_call!(
            self.0,
            getEntryPointCodeBlob(entry_point_index, target_index, &mut blob)
        ))?;

        Ok(Blob(sys::IBlob::from_raw(blob)))
    }

    #[inline]
    pub unsafe fn get_entry_point_host_callable(
        &mut self,
        entry_point_index: i32,
        target_index: i32,
    ) -> utils::Result<SharedLibrary> {
        let mut library = ptr::null_mut();
        utils::result_from_ffi(vtable_call!(
            self.0,
            getEntryPointHostCallable(entry_point_index, target_index, &mut library)
        ))?;
        Ok(SharedLibrary(sys::ISharedLibrary::from_raw(library)))
    }

    #[inline]
    pub unsafe fn get_target_code_blob(&mut self, target_index: i32) -> utils::Result<Blob> {
        let mut blob = ptr::null_mut();
        utils::result_from_ffi(vtable_call!(
            self.0,
            getTargetCodeBlob(target_index, &mut blob)
        ))?;

        Ok(Blob(sys::IBlob::from_raw(blob)))
    }

    #[inline]
    pub unsafe fn get_target_host_callable(
        &mut self,
        target_index: i32,
    ) -> utils::Result<SharedLibrary> {
        let mut library = ptr::null_mut();

        utils::result_from_ffi(vtable_call!(
            self.0,
            getTargetHostCallable(target_index, &mut library)
        ))?;

        Ok(SharedLibrary(sys::ISharedLibrary::from_raw(library)))
    }

    #[inline]
    pub unsafe fn get_compile_request_code(&mut self) -> &[u8] {
        let mut len = 0;
        let data = vtable_call!(self.0, getCompileRequestCode(&mut len));
        slice::from_raw_parts(data.cast(), len)
    }

    #[inline]
    pub unsafe fn get_compile_request_result_as_file_system(&mut self) -> MutableFileSystem {
        let system = vtable_call!(self.0, getCompileRequestResultAsFileSystem());
        MutableFileSystem(sys::IMutableFileSystem::from_raw(system))
    }

    #[inline]
    pub unsafe fn get_container_code(&mut self) -> utils::Result<Blob> {
        let mut blob = ptr::null_mut();
        utils::result_from_ffi(vtable_call!(self.0, getContainerCode(&mut blob)))?;
        Ok(Blob(sys::IBlob::from_raw(blob)))
    }

    #[inline]
    pub unsafe fn load_repro(
        &mut self,
        file_system: &mut FileSystem,
        data: *const c_void,
        size: usize,
    ) -> utils::Result<()> {
        utils::result_from_ffi(vtable_call!(
            self.0,
            loadRepro(&mut file_system.0, data, size)
        ))
    }

    #[inline]
    pub unsafe fn save_repro(&mut self) -> utils::Result<Blob> {
        let mut blob = ptr::null_mut();
        utils::result_from_ffi((vtable_call!(self.0, saveRepro(&mut blob))))?;
        Ok(Blob(sys::IBlob::from_raw(blob)))
    }

    #[inline]
    pub unsafe fn enable_repro_capture(&mut self) -> utils::Result<()> {
        utils::result_from_ffi(vtable_call!(self.0, enableReproCapture()))
    }

    #[inline]
    pub unsafe fn get_program(&mut self) -> utils::Result<ComponentType> {
        let mut component = ptr::null_mut();
        utils::result_from_ffi(vtable_call!(self.0, getProgram(&mut component)))?;
        Ok(ComponentType(sys::IComponentType::from_raw(component)))
    }

    #[inline]
    pub unsafe fn get_entry_point(
        &mut self,
        entry_point_index: i64,
    ) -> utils::Result<ComponentType> {
        let mut component = ptr::null_mut();
        utils::result_from_ffi(vtable_call!(
            self.0,
            getEntryPoint(entry_point_index, &mut component)
        ))?;
        Ok(ComponentType(sys::IComponentType::from_raw(component)))
    }

    #[inline]
    pub unsafe fn get_module(&mut self, translation_unit_index: i64) -> utils::Result<Module> {
        let mut module = ptr::null_mut();
        utils::result_from_ffi(vtable_call!(
            self.0,
            getModule(translation_unit_index, &mut module)
        ))?;
        Ok(Module(sys::IModule::from_raw(module)))
    }

    #[inline]
    pub unsafe fn get_session(&mut self) -> utils::Result<Session> {
        let mut session = ptr::null_mut();
        utils::result_from_ffi(vtable_call!(self.0, getSession(&mut session)))?;
        Ok(Session(sys::ISession::from_raw(session)))
    }

    //TODO: replace slang reflection with Reflection, I can't find it in header
    #[inline]
    pub unsafe fn get_reflection(&mut self) -> *mut SlangReflection {
        vtable_call!(self.0, getReflection())
    }

    #[inline]
    pub unsafe fn set_command_line_compiler_mode(&mut self) {
        vtable_call!(self.0, setCommandLineCompilerMode())
    }

    #[inline]
    pub unsafe fn add_target_capability(
        &mut self,
        target_index: i64,
        capability: CapabilityID,
    ) -> utils::Result<()> {
        utils::result_from_ffi(vtable_call!(
            self.0,
            addTargetCapability(target_index, capability.0)
        ))
    }

    #[inline]
    pub unsafe fn get_program_with_entry_points(&mut self) -> utils::Result<ComponentType> {
        let mut out_program = ptr::null_mut();
        utils::result_from_ffi(vtable_call!(
            self.0,
            getProgramWithEntryPoints(&mut out_program)
        ))?;

        Ok(ComponentType(sys::IComponentType::from_raw(out_program)))
    }

    #[inline]
    pub unsafe fn is_parameter_location_used(
        &self,
        entry_point_index: i64,
        target_index: i64,
        category: ParameterCategory,
        space_index: u64,
        register_index: u64,
    ) -> utils::Result<bool> {
        let mut used = false;
        utils::result_from_ffi(vtable_call!(
            self.0,
            isParameterLocationUsed(
                entry_point_index,
                target_index,
                category.0,
                space_index,
                register_index,
                &mut used
            )
        ))?;
        Ok(used)
    }

    #[inline]
    pub unsafe fn set_target_line_directive_mode(
        &mut self,
        target_index: i64,
        mode: SourceLanguage,
    ) {
        vtable_call!(
            self.0,
            setTargetLineDirectiveMode(target_index, mode.0 as _)
        )
    }

    #[inline]
    pub unsafe fn set_target_force_glsl_scalar_buffer_layout(
        &mut self,
        target_index: i64,
        force_scalar_layout: bool,
    ) {
        vtable_call!(
            self.0,
            setTargetForceGLSLScalarBufferLayout(target_index, force_scalar_layout)
        )
    }

    #[inline]
    pub unsafe fn override_diagnostic_severity(
        &mut self,
        message_id: i64,
        override_severity: Severity,
    ) {
        vtable_call!(
            self.0,
            overrideDiagnosticSeverity(message_id, override_severity.0)
        )
    }

    #[inline]
    pub unsafe fn get_diagnostic_flags(&mut self) -> DiagnosticFlags {
        mem::transmute(vtable_call!(self.0, getDiagnosticFlags()))
    }

    #[inline]
    pub unsafe fn set_diagnostic_flags(&mut self, flags: DiagnosticFlags) {
        vtable_call!(self.0, setDiagnosticFlags(mem::transmute(flags)))
    }

    #[inline]
    pub unsafe fn set_debug_info_format(&mut self, debug_format: DebugInfoFormat) {
        vtable_call!(self.0, setDebugInfoFormat(mem::transmute(debug_format)))
    }

    #[inline]
    pub unsafe fn set_enable_effect_annotations(&mut self, value: bool) {
        vtable_call!(self.0, setEnableEffectAnnotations(value))
    }

    #[inline]
    pub unsafe fn set_report_downstream_time(&mut self, value: bool) {
        vtable_call!(self.0, setReportDownstreamTime(value))
    }

    #[inline]
    pub unsafe fn set_report_perf_benchmark(&mut self, value: bool) {
        vtable_call!(self.0, setReportPerfBenchmark(value))
    }

    #[inline]
    pub unsafe fn set_skip_spirv_validation(&mut self, value: bool) {
        vtable_call!(self.0, setSkipSPIRVValidation(value))
    }
}
