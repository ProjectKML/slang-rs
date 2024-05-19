use std::slice;

use slang::{
    CompileTarget, FileSystem, GlobalSession, SessionDescBuilder, TargetDescBuilder, TargetFlags,
};

struct MyFileSystem;

impl FileSystem for MyFileSystem {
    fn load_file(&mut self, path: &str) -> std::io::Result<String> {
        Ok(r#"

        "#
        .to_owned())
    }
}

fn main() {
    let global_session = GlobalSession::new().unwrap();

    let target_desc = TargetDescBuilder::default()
        .format(CompileTarget::SPIRV)
        .profile(global_session.find_profile("spirv_1_4"))
        .flags(TargetFlags::GENERATE_SPIRV_DIRECTLY)
        .force_glsl_scalar_buffer_layout(true);

    let session_desc = SessionDescBuilder::default()
        .targets(slice::from_ref(&target_desc))
        .file_system(MyFileSystem);

    let mut session = global_session.create_session(session_desc).unwrap();
    //let module = session.load_module("example").unwrap();
}
