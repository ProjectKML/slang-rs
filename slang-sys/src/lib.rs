#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

mod interface;

pub use interface::*;

use std::ffi::{c_char, c_int, c_void};

include!("../gen/bindings.rs");

interface!(IUnknown, ISlangUnknown, [0x00000000, 0x0000, 0x0000, [0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x46]], {
	queryInterface: unsafe extern "C" fn(*mut c_void, *const SlangUUID, *mut *mut ::std::os::raw::c_void) -> SlangResult,
	addRef: unsafe extern "C" fn(*mut c_void) -> u32,
	release: unsafe extern "C" fn(*mut c_void) -> u32,
});

interface!(IGlobalSession, [0xc140b5fd, 0x0c78, 0x452e, [0xba, 0x7c, 0x1a, 0x1e, 0x70, 0xc7, 0xf7, 0x1c]], {
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

interface!(ISession, [0x67618701, 0xd116, 0x468f, [0xab, 0x3b, 0x47, 0x4b, 0xed, 0xce, 0xe, 0x3d]], {

});

interface!(ICompileRequest, [0x96d33993, 0x317c, 0x4db5, [0xaf, 0xd8, 0x66, 0x6e, 0xe7, 0x72, 0x48, 0xe2]], {
	setFileSystem: unsafe extern "stdcall" fn(*mut c_void, fileSystem: *mut ISlangFileSystem),
	setCompileFlags: unsafe extern "stdcall" fn(*mut c_void, flags: SlangCompileFlags),
	getCompileFlags: unsafe extern "stdcall" fn(*mut c_void) -> SlangCompileFlags,
	setDumpIntermediates: unsafe extern "stdcall" fn(*mut c_void, enable: c_int),
	setDumpIntermediatePrefix: unsafe extern "stdcall" fn(*mut c_void, prefix: *const c_char),
	setLineDirectiveMode: unsafe extern "stdcall" fn(*mut c_void, mode: SlangLineDirectiveMode),
	setCodeGenTarget: unsafe extern "stdcall" fn(*mut c_void, target: SlangCompileTarget),
	addCodeGenTarget: unsafe extern "stdcall" fn(*mut c_void, target: SlangCompileTarget) -> i32,
	setTargetProfile: unsafe extern "stdcall" fn(*mut c_void, target_index: i32, profile: SlangProfileID),
	setTargetFlags: unsafe extern "stdcall" fn(*mut c_void, target_index: i32, flags: SlangTargetFlags),
	setTargetFloatingPointMode: unsafe extern "stdcall" fn(*mut c_void, target_index: i32, mode: SlangFloatingPointMode),
	setTargetMatrixLayoutMode: unsafe extern "stdcall" fn(*mut c_void, target_index: i32, mode: SlangMatrixLayoutMode),
	setMatrixLayoutMode: unsafe extern "stdcall" fn(*mut c_void, mode: SlangMatrixLayoutMode),
	setDebugInfoLevel: unsafe extern "stdcall" fn(*mut c_void, level: SlangDebugInfoLevel),
	setOptimizationLevel: unsafe extern "stdcall" fn(*mut c_void, level: SlangOptimizationLevel),
	setOutputContainerFormat: unsafe extern "stdcall" fn(*mut c_void, format: SlangContainerFormat),
	setPassThrough: unsafe extern "stdcall" fn(*mut c_void, passThrough: SlangPassThrough),
	setDiagnosticCallback: unsafe extern "stdcall" fn(*mut c_void, callback: SlangDiagnosticCallback, userData: *const c_void),
	setWriter: unsafe extern "stdcall" fn(*mut c_void, channel: SlangWriterChannel, writer: *mut ISlangWriter),
	getWriter: unsafe extern "stdcall" fn(*mut c_void, channel: SlangWriterChannel) -> *mut ISlangWriter,
	addSearchPath: unsafe extern "stdcall" fn(*mut c_void, searchDir: *const c_char),
	addPreprocessorDefine: unsafe extern "stdcall" fn(*mut c_void, key: *const c_char, value: *const c_char),
	processCommandLineArguments: unsafe extern "stdcall" fn(*mut c_void, args: *const *const c_char, argCount: c_int) -> SlangResult,
	addTranslationUnit: unsafe extern "stdcall" fn(*mut c_void, language: SlangSourceLanguage, name: *const c_char) -> c_int,
	setDefaultModuleName: unsafe extern "stdcall" fn(*mut c_void, defaultModuleName: *const c_char),
	addTranslationUnitPreprocessorDefine: unsafe extern "stdcall" fn(*mut c_void, translationUnitIndex: c_int, key: *const c_char, value: *const c_char),
	addTranslationUnitSourceFile: unsafe extern "stdcall" fn(*mut c_void, translationUnitIndex: c_int, path: *const c_char),
	addTranslationUnitSourceString: unsafe extern "stdcall" fn(*mut c_void, translationUnitIndex: c_int, path: *const c_char, source: *const c_char),
	addLibraryReference: unsafe extern "stdcall" fn(*mut c_void, basePath: *const c_char, libData: *const c_void, libDataSize: usize) -> SlangResult,
	addTranslationUnitSourceStringSpan: unsafe extern "stdcall" fn(*mut c_void, translationUnitIndex: c_int, path: *const c_char, sourceBegin: *const c_char, sourceEnd: *const c_char),
	addTranslationUnitSourceBlob: unsafe extern "stdcall" fn(*mut c_void, translationUnitIndex: c_int, path: *const c_char, sourceBlob: *mut ISlangBlob),
	addEntryPoint: unsafe extern "stdcall" fn(*mut c_void, translationUnitIndex: c_int, name: *const c_char, stage: SlangStage) -> c_int,
	addEntryPointEx: unsafe extern "stdcall" fn(*mut c_void, translationUnitIndex: c_int, name: *const c_char, stage: SlangStage, genericArgCount: c_int, genericArgs: *const *const c_char) -> c_int,
	setGlobalGenericArgs: unsafe extern "stdcall" fn(*mut c_void, genericArgCount: c_int, genericArgs: *const *const c_char) -> SlangResult,
	setTypeNameForGlobalExistentialTypeParam: unsafe extern "stdcall" fn(*mut c_void, slotIndex: c_int, typeName: *const c_char) -> SlangResult,
	setTypeNameForEntryPointExistentialTypeParam: unsafe extern "stdcall" fn(*mut c_void, entryPointIndex: c_int, slotIndex: c_int, typeName: *const c_char) -> SlangResult,
	setAllowGLSLInput: unsafe extern "stdcall" fn(*mut c_void, value: bool),
	compile: unsafe extern "stdcall" fn(*mut c_void) -> SlangResult,
	getDiagnosticOutput: unsafe extern "stdcall" fn(*mut c_void) -> *const c_char,
	getDiagnosticOutputBlob: unsafe extern "stdcall" fn(*mut c_void, outBlob: *mut *mut ISlangBlob) -> SlangResult,
	getDependencyFileCount: unsafe extern "stdcall" fn(*mut c_void) -> c_int,
	getDependencyFilePath: unsafe extern "stdcall" fn(*mut c_void, index: c_int) -> *const c_char,
	getTranslationUnitCount: unsafe extern "stdcall" fn(*mut c_void) -> c_int,
	getEntryPointSource: unsafe extern "stdcall" fn(*mut c_void, entryPointIndex: c_int) -> *const c_char,
	getEntryPointCode: unsafe extern "stdcall" fn(*mut c_void, entryPointIndex: c_int, outSize: *mut usize) -> *const c_void,
	getEntryPointCodeBlob: unsafe extern "stdcall" fn(*mut c_void, entryPointIndex: c_int, targetIndex: c_int, outBlob: *mut *mut ISlangBlob) -> SlangResult,
	getEntryPointHostCallable: unsafe extern "stdcall" fn(*mut c_void, entryPointIndex: c_int, targetIndex: c_int, outSharedLibrary: *mut *mut ISlangSharedLibrary) -> SlangResult,
	getTargetCodeBlob: unsafe extern "stdcall" fn(*mut c_void, targetIndex: c_int, outBlob: *mut *mut ISlangBlob) -> SlangResult,
	getTargetHostCallable: unsafe extern "stdcall" fn(*mut c_void, targetIndex: c_int, outSharedLibrary: *mut *mut ISlangSharedLibrary) -> SlangResult,
	getCompileRequestCode: unsafe extern "stdcall" fn(*mut c_void, outSize: *mut usize) -> *const c_void,
	getCompileRequestResultAsFileSystem: unsafe extern "stdcall" fn(*mut c_void) -> *mut ISlangMutableFileSystem,
	getCompileRequestContainerBlob: unsafe extern "stdcall" fn(*mut c_void, outBlob: *mut *mut ISlangBlob) -> SlangResult,
	getContainerCode: unsafe extern "stdcall" fn(*mut c_void, outBlob: *mut *mut ISlangBlob) -> SlangResult,
	loadRepro: unsafe extern "stdcall" fn(*mut c_void, fileSystem: *mut ISlangFileSystem, data: *const c_void, size: usize) -> SlangResult,
	saveRepro: unsafe extern "stdcall" fn(*mut c_void, outBlob: *mut *mut ISlangBlob) -> SlangResult,
	enableReproCapture: unsafe extern "stdcall" fn(*mut c_void) -> SlangResult,
	getLinkedProgram: unsafe extern "stdcall" fn(*mut c_void, outProgram: *mut *mut slang_IComponentType) -> SlangResult,
	getProgram: unsafe extern "stdcall" fn(*mut c_void, outProgram: *mut *mut slang_IComponentType) -> SlangResult,
	getEntryPoint: unsafe extern "stdcall" fn(*mut c_void, entryPointIndex: SlangInt, outEntryPoint: *mut *mut slang_IComponentType) -> SlangResult,
	getModule: unsafe extern "stdcall" fn(*mut c_void, translationUnitIndex: SlangInt, outModule: *mut *mut slang_IModule) -> SlangResult,
	getSession: unsafe extern "stdcall" fn(*mut c_void, outSession: *mut *mut slang_ISession) -> SlangResult,
	getReflection: unsafe extern "stdcall" fn(*mut c_void) -> *mut SlangReflection,
	setCommandLineCompilerMode: unsafe extern "stdcall" fn(*mut c_void),
	addTargetCapability: unsafe extern "stdcall" fn(*mut c_void, targetIndex: SlangInt, capability: SlangCapabilityID) -> SlangResult,
	getProgramWithEntryPoints: unsafe extern "stdcall" fn(*mut c_void, outProgram: *mut *mut slang_IComponentType) -> SlangResult,
	isParameterLocationUsed: unsafe extern "stdcall" fn(*mut c_void, entryPointIndex: SlangInt, targetIndex: SlangInt, category: SlangParameterCategory, spaceIndex: SlangUInt, registerIndex: SlangUInt, outUsed: *mut bool) -> SlangResult,
	setTargetLineDirectiveMode: unsafe extern "stdcall" fn(*mut c_void, targetIndex: SlangInt, mode: SlangLineDirectiveMode),
	setTargetForceGLSLScalarBufferLayout: unsafe extern "stdcall" fn(*mut c_void, targetIndex: SlangInt, forceScalarLayout: bool),
	overrideDiagnosticSeverity: unsafe extern "stdcall" fn(*mut c_void, messageID: SlangInt, overrideSeverity: SlangSeverity),
	getDiagnosticFlags: unsafe extern "stdcall" fn(*mut c_void) -> SlangDiagnosticFlags,
	setDiagnosticFlags: unsafe extern "stdcall" fn(*mut c_void, flags: SlangDiagnosticFlags),
	setDebugInfoFormat: unsafe extern "stdcall" fn(*mut c_void, debugFormat: SlangDebugInfoFormat),
	setEnableEffectAnnotations: unsafe extern "stdcall" fn(*mut c_void, value: bool),
	setReportDownstreamTime: unsafe extern "stdcall" fn(*mut c_void, value: bool),
	setReportPerfBenchmark: unsafe extern "stdcall" fn(*mut c_void, value: bool),
	setSkipSPIRVValidation: unsafe extern "stdcall" fn(*mut c_void, value: bool),
});

interface!(IWriter, ISlangWriter, [0xec457f0e, 0x9add, 0x4e6b, [0x85, 0x1c, 0xd7, 0xfa, 0x71, 0x6d, 0x15, 0xfd]], {

});

interface!(IBlob, [0x8BA5FB08, 0x5195, 0x40e2, [0xAC, 0x58, 0x0D, 0x98, 0x9C, 0x3A, 0x01, 0x02]], {
	getBufferPointer: unsafe extern "stdcall" fn(*mut c_void) -> *const c_void,
	getBufferSize: unsafe extern "stdcall" fn(*mut c_void) -> usize,
});

interface!(ISharedLibrary, ISlangSharedLibrary, [0x9c9d5bc5, 0xeb61, 0x496f, [0x80, 0xd7, 0xd1, 0x47, 0xc4, 0xa2, 0x37, 0x30]], {

});

interface!(IMutableFileSystem, ISlangMutableFileSystem, [0xa058675c, 0x1d65, 0x452a, [0x84, 0x58, 0xcc, 0xde, 0xd1, 0x42, 0x71, 0x5]], {

});

interface!(IComponentType, [0x5bc42be8, 0x5c50, 0x4929, [0x9e, 0x5e, 0xd1, 0x5e, 0x7c, 0x24, 0x1, 0x5f]], {

});

interface!(IModule, [0xc720e64, 0x8722, 0x4d31, [0x89, 0x90, 0x63, 0x8a, 0x98, 0xb1, 0xc2, 0x79]], {
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

interface!(IEntryPoint, [0x8f241361, 0xf5bd, 0x4ca0, [0xa3, 0xac, 0x2, 0xf7, 0xfa, 0x24, 0x2, 0xb8 ]], {

});