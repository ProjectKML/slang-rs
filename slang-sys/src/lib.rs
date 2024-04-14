#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

mod interface;

pub use interface::*;

use std::ffi::{c_char, c_int, c_void};

include!("../gen/bindings.rs");

interface!(IUnknown, ISlangUnknown, [0xc140b5fd, 0x0c78, 0x452e, [0xba, 0x7c, 0x1a, 0x1e, 0x70, 0xc7, 0xf7, 0x1c]], {
	queryInterface: unsafe extern "C" fn(*mut ISlangUnknown, *const SlangUUID, *mut *mut ::std::os::raw::c_void) -> SlangResult,
	addRef: unsafe extern "C" fn(*mut ISlangUnknown) -> u32,
	release: unsafe extern "C" fn(*mut ISlangUnknown) -> u32,
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