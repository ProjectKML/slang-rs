#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use std::ffi::{c_char, c_int, c_void};

pub use interface::*;

mod interface;

include!("../gen/bindings.rs");

interface!(IUnknown, ISlangUnknown, [0x00000000, 0x0000, 0x0000, [0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46]], {
	queryInterface: unsafe extern "stdcall" fn(*mut c_void, *const SlangUUID, *mut *mut ::std::os::raw::c_void) -> SlangResult,
	addRef: unsafe extern "stdcall" fn(*mut c_void) -> u32,
	release: unsafe extern "stdcall" fn(*mut c_void) -> u32,
});

interface!(IGlobalSession, slang_IGlobalSession, [0xc140b5fd, 0x0c78, 0x452e, [0xba, 0x7c, 0x1a, 0x1e, 0x70, 0xc7, 0xf7, 0x1c]], {
	createSession: unsafe extern "stdcall" fn(*mut c_void, desc: *const slang_SessionDesc, outSession: *mut *mut slang_ISession) -> SlangResult,
	findProfile: unsafe extern "stdcall" fn(*mut c_void, name: *const c_char) -> SlangProfileID,
	setDownstreamCompilerPath: unsafe extern "stdcall" fn(*mut c_void, passThrough: SlangPassThrough, path: *const c_char),
	setDownstreamCompilerPrelude: unsafe extern "stdcall" fn(*mut c_void, passThrough: SlangPassThrough, preludeText: *const c_char),
	getDownstreamCompilerPrelude: unsafe extern "stdcall" fn(*mut c_void, passThrough: SlangPassThrough, outPrelude: *mut *mut ISlangBlob),
	getBuildTagString: unsafe extern "stdcall" fn(*mut c_void) -> *const c_char,
	setDefaultDownstreamCompiler: unsafe extern "stdcall" fn(*mut c_void, sourceLanguage: SlangSourceLanguage, defaultCompiler: SlangPassThrough) -> SlangResult,
	getDefaultDownstreamCompiler: unsafe extern "stdcall" fn(*mut c_void, sourceLanguage: SlangSourceLanguage) -> SlangPassThrough,
	setLanguagePrelude: unsafe extern "stdcall" fn(*mut c_void, sourceLanguage: SlangSourceLanguage, preludeText: *const c_char),
	getLanguagePrelude: unsafe extern "stdcall" fn(*mut c_void, sourceLanguage: SlangSourceLanguage, outPrelude: *mut *mut ISlangBlob),
	createCompileRequest: unsafe extern "stdcall" fn(*mut c_void, *mut *mut slang_ICompileRequest) -> SlangResult,
	addBuiltins: unsafe extern "stdcall" fn(*mut c_void, sourcePath: *const c_char, sourceString: *const c_char),
	setSharedLibraryLoader: unsafe extern "stdcall" fn(*mut c_void, loader: *mut ISlangSharedLibraryLoader),
	getSharedLibraryLoader: unsafe extern "stdcall" fn(*mut c_void) -> *mut ISlangSharedLibraryLoader,
	checkCompileTargetSupport: unsafe extern "stdcall" fn(*mut c_void, target: SlangCompileTarget) -> SlangResult,
	checkPassThroughSupport: unsafe extern "stdcall" fn(*mut c_void, passThrough: SlangPassThrough) -> SlangResult,
	compileStdLib: unsafe extern "stdcall" fn(*mut c_void, flags: slang_CompileStdLibFlags) -> SlangResult,
	loadStdLib: unsafe extern "stdcall" fn(*mut c_void, stdLib: *const c_void, stdLibSizeInBytes: usize) -> SlangResult,
	saveStdLib: unsafe extern "stdcall" fn(*mut c_void, archiveType: SlangArchiveType, outBlob: *mut *mut ISlangBlob) -> SlangResult,
	findCapability: unsafe extern "stdcall" fn(*mut c_void, name: *const c_char) -> SlangCapabilityID,
	setDownstreamCompilerForTransition: unsafe extern "stdcall" fn(*mut c_void, source: SlangCompileTarget, target: SlangCompileTarget, compiler: SlangPassThrough),
	getDownstreamCompilerForTransition: unsafe extern "stdcall" fn(*mut c_void, source: SlangCompileTarget, target: SlangCompileTarget) -> SlangPassThrough,
	getCompilerElapsedTime: unsafe extern "stdcall" fn(*mut c_void, outTotalTime: *mut f64, outDownstreamTime: *mut f64),
	setSPIRVCoreGrammar: unsafe extern "stdcall" fn(*mut c_void, jsonPath: *const c_char) -> SlangResult,
	parseCommandLineArguments: unsafe extern "stdcall" fn(*mut c_void, argc: c_int, argv: *const *const c_char, outSessionDesc: *mut slang_SessionDesc, outAuxAllocation: *mut *mut ISlangUnknown) -> SlangResult,
	getSessionDescDigest: unsafe extern "stdcall" fn(*mut c_void, sessionDesc: *const slang_SessionDesc, outBlob: *mut *mut ISlangBlob) -> SlangResult,
});

interface!(ISession, slang_ISession, [0x67618701, 0xd116, 0x468f, [0xab, 0x3b, 0x47, 0x4b, 0xed, 0xce, 0xe, 0x3d]], {
	getGlobalSession: unsafe extern "stdcall" fn(*mut c_void) -> *mut slang_IGlobalSession,
	loadModule: unsafe extern "stdcall" fn(*mut c_void, moduleName: *const c_char, outDiagnostics: *mut *mut slang_IBlob) -> *mut slang_IModule,
	loadModuleFromSource: unsafe extern "stdcall" fn(*mut c_void, moduleName: *const c_char, path: *const c_char, source: *mut slang_IBlob, outDiagnostics: *mut *mut slang_IBlob) -> *mut slang_IModule,
	createCompositeComponentType: unsafe extern "stdcall" fn(*mut c_void, componentTypes: *const *const slang_IComponentType, componentTypeCount: SlangInt, outCompositeComponentType: *mut *mut slang_IComponentType, outDiagnostics: *mut *mut ISlangBlob) -> SlangResult,
	specializeType: unsafe extern "stdcall" fn(*mut c_void, type_: *mut slang_TypeReflection, specializationArgs: *const slang_SpecializationArg, specializationArgCount: SlangInt, outDiagnostics: *mut *mut ISlangBlob) -> *mut slang_TypeReflection,
	getTypeLayout: unsafe extern "stdcall" fn(*mut c_void, type_: *mut slang_TypeReflection, targetIndex: SlangInt, rules: slang_LayoutRules, outDiagnostics: *mut *mut ISlangBlob) -> *mut slang_TypeLayoutReflection,
	getDynamicType: unsafe extern "stdcall" fn(*mut c_void) -> *mut slang_TypeReflection,
	getTypeRTTIMangledName: unsafe extern "stdcall" fn(*mut c_void, type_: *mut slang_TypeReflection, outNameBlob: *mut *mut ISlangBlob) -> SlangResult,
	getTypeConformanceWitnessMangledName: unsafe extern "stdcall" fn(*mut c_void, type_: *mut slang_TypeReflection, interfaceType: *mut slang_TypeReflection, outNameBlob: *mut *mut ISlangBlob) -> SlangResult,
	getTypeConformanceWitnessSequentialID: unsafe extern "stdcall" fn(*mut c_void, type_: *mut slang_TypeReflection, interfaceType: *mut slang_TypeReflection, outId: *mut u32) -> SlangResult,
	createCompileRequest: unsafe extern "stdcall" fn(*mut c_void, outCompileRequest: *mut *mut SlangCompileRequest) -> SlangResult,
	createTypeConformanceComponentType: unsafe extern "stdcall" fn(*mut c_void, type_: *mut slang_TypeReflection, interfaceType: *mut slang_TypeReflection, outConformance: *mut *mut slang_ITypeConformance, conformanceIdOverride: SlangInt, outDiagnostics: *mut *mut ISlangBlob) -> SlangResult,
	loadModuleFromIRBlob: unsafe extern "stdcall" fn(*mut c_void, moduleName: *const c_char, path: *const c_char, source: *mut slang_IBlob, outDiagnostics: *mut *mut slang_IBlob) -> *mut slang_IModule,
	getLoadedModuleCount: unsafe extern "stdcall" fn(*mut c_void) -> SlangInt,
	getLoadedModule: unsafe extern "stdcall" fn(*mut c_void, index: SlangInt) -> *mut slang_IModule,
	isBinaryModuleUpToDate: unsafe extern "stdcall" fn(*mut c_void, modulePath: *const c_char, binaryModuleBlob: *mut slang_IBlob) -> bool,
	loadModuleFromSourceString: unsafe extern "stdcall" fn(*mut c_void, moduleName: *const c_char, path: *const c_char, string: *const c_char, outDiagnostics: *mut *mut slang_IBlob) -> *mut slang_IModule,
});

interface!(IWriter, ISlangWriter, [0xec457f0e, 0x9add, 0x4e6b, [0x85, 0x1c, 0xd7, 0xfa, 0x71, 0x6d, 0x15, 0xfd]], {

});

interface!(IBlob, slang_IBlob, [0x8BA5FB08, 0x5195, 0x40e2, [0xAC, 0x58, 0x0D, 0x98, 0x9C, 0x3A, 0x01, 0x02]], {
	getBufferPointer: unsafe extern "stdcall" fn(*mut c_void) -> *const c_void,
	getBufferSize: unsafe extern "stdcall" fn(*mut c_void) -> usize,
});

interface!(ICastable, ISlangCastable, [0x87ede0e1, 0x4852, 0x44b0, [0x8b, 0xf2, 0xcb, 0x31, 0x87, 0x4d, 0xe2, 0x39]], {
	castAs: unsafe extern "stdcall" fn(*mut c_void, guid: &SlangUUID) -> *mut c_void,
});

interface!(IMutableFileSystem, ISlangMutableFileSystem, [0xa058675c, 0x1d65, 0x452a, [0x84, 0x58, 0xcc, 0xde, 0xd1, 0x42, 0x71, 0x5]], { //TODO: inheritance

});

interface!(ISharedLibrary, ISlangSharedLibrary, [0x70dbc7c4, 0xdc3b, 0x4a07, [0xae, 0x7e, 0x75, 0x2a, 0xf6, 0xa8, 0x15, 0x55]]: ICastable, {
	findSymbolAddressByName: unsafe extern "stdcall" fn(*mut c_void, name: *mut c_char) -> *mut c_void,
});

interface!(IComponentType, slang_IComponentType, [0x5bc42be8, 0x5c50, 0x4929, [0x9e, 0x5e, 0xd1, 0x5e, 0x7c, 0x24, 0x1, 0x5f]], {
	getSession: unsafe extern "stdcall" fn(*mut c_void) -> *mut slang_ISession,
	getLayout: unsafe extern "stdcall" fn(*mut c_void, target_index: SlangInt, out_diagnostics: *mut *mut slang_IBlob) -> *mut slang_ProgramLayout,
	getSpecializationParamCount: unsafe extern "stdcall" fn(*mut c_void) -> SlangInt,
	getEntryPointCode: unsafe extern "stdcall" fn(*mut c_void, entry_point_index: SlangInt, target_index: SlangInt, out_code: *mut *mut slang_IBlob, out_diagnostics: *mut *mut slang_IBlob) -> SlangResult,
	getResultAsFileSystem: unsafe extern "stdcall" fn(*mut c_void, entry_point_index: SlangInt, target_index: SlangInt, out_file_system: *mut *mut ISlangMutableFileSystem) -> SlangResult,
	getEntryPointHash: unsafe extern "stdcall" fn(*mut c_void, entry_point_index: SlangInt, target_index: SlangInt, out_hash: *mut *mut slang_IBlob),
	specialize: unsafe extern "stdcall" fn(*mut c_void, specialization_args: *const slang_SpecializationArg, specialization_arg_count: SlangInt, out_specialized_component_type: *mut *mut slang_IComponentType, out_diagnostics: *mut *mut ISlangBlob) -> SlangResult,
	link: unsafe extern "stdcall" fn(*mut c_void, out_linked_component_type: *mut *mut slang_IComponentType, out_diagnostics: *mut *mut ISlangBlob) -> SlangResult,
	getEntryPointHostCallable: unsafe extern "stdcall" fn(*mut c_void, entry_point_index: SlangInt, target_index: SlangInt, out_shared_library: *mut *mut ISlangSharedLibrary, out_diagnostics: *mut *mut slang_IBlob) -> SlangResult,
	renameEntryPoint: unsafe extern "stdcall" fn(*mut c_void, new_name: *const c_char, out_entry_point: *mut *mut slang_IComponentType) -> SlangResult,
	linkWithOptions: unsafe extern "stdcall" fn(*mut c_void, out_linked_component_type: *mut *mut slang_IComponentType, compiler_option_entry_count: u32, compiler_option_entries: *mut slang_CompilerOptionEntry, out_diagnostics: *mut *mut ISlangBlob) -> SlangResult,
});

interface!(IModule, slang_IModule, [0xc720e64, 0x8722, 0x4d31, [0x89, 0x90, 0x63, 0x8a, 0x98, 0xb1, 0xc2, 0x79]]: IComponentType, {
	findEntryPointByName: unsafe extern "stdcall" fn(*mut c_void, name: *const c_char, outEntryPoint: *mut *mut slang_IEntryPoint) -> SlangResult,
	getDefinedEntryPointCount: unsafe extern "stdcall" fn(*mut c_void) -> SlangInt32,
	getDefinedEntryPoint: unsafe extern "stdcall" fn(*mut c_void, index: SlangInt32, outEntryPoint: *mut *mut slang_IEntryPoint) -> SlangResult,
	serialize: unsafe extern "stdcall" fn(*mut c_void, outSerializedBlob: *mut *mut ISlangBlob) -> SlangResult,
	writeToFile: unsafe extern "stdcall" fn(*mut c_void, fileName: *const c_char) -> SlangResult,
	getName: unsafe extern "stdcall" fn(*mut c_void) -> *const c_char,
	getFilePath: unsafe extern "stdcall" fn(*mut c_void) -> *const c_char,
	getUniqueIdentity: unsafe extern "stdcall" fn(*mut c_void) -> *const c_char,
	findAndCheckEntryPoint: unsafe extern "stdcall" fn(*mut c_void, name: *const c_char, stage: SlangStage, outEntryPoint: *mut *mut slang_IEntryPoint, outDiagnostics: *mut *mut ISlangBlob) -> SlangResult,
});

interface!(IEntryPoint, slang_IEntryPoint, [0x8f241361, 0xf5bd, 0x4ca0, [0xa3, 0xac, 0x2, 0xf7, 0xfa, 0x24, 0x2, 0xb8 ]]: IComponentType, {

});