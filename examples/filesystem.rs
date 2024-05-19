use std::{ops::Deref, slice};

use slang::{
    CompileTarget, FileSystem, GlobalSession, SessionDescBuilder, TargetDescBuilder, TargetFlags,
};

struct MyFileSystem;

impl FileSystem for MyFileSystem {
    fn load_file(&mut self, path: &str) -> Option<String> {
        if path.ends_with(".slang") {
            Some(
                r#"
[shader("compute")]
[numthreads(1, 1, 1)]
void main() {
}
            "#
                .to_owned(),
            )
        } else {
            None
        }
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
    let mut module = session.load_module("example").unwrap();

    let entry_point = module.find_entry_point_by_name("main").unwrap();

    let mut program = session
        .create_composite_component_type(&[module.deref().clone(), entry_point.deref().clone()])
        .unwrap();
    let linked_program = program.link().unwrap();
    let code = linked_program.get_entry_point_code(0, 0).unwrap();
    println!("{:?}", code.as_slice());
}
