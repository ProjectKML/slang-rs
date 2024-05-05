use std::mem;

use slang::{Blob, CompileTarget, GlobalSession, SessionDesc, TargetDesc, TargetFlags};

fn main() {
    let global_session = GlobalSession::new().unwrap();

    let target_desc = TargetDesc {
        structure_size: mem::size_of::<TargetDesc>(),
        format: CompileTarget::SPIRV,
        profile: global_session.find_profile("spirv_1_4"),
        flags: TargetFlags::GENERATE_SPIRV_DIRECTLY,
        force_glsl_scalar_buffer_layout: true,
        ..Default::default()
    };

    let session_desc = SessionDesc {
        targets: &target_desc,
        target_count: 1,
        ..Default::default()
    };

    let mut session = unsafe { global_session.create_session(&session_desc) }.unwrap();

    let mut blob = Blob::from(
        r#"
struct MyValue {
    uint value;
}

[[vk::push_constant]] struct PushConstants {
    MyValue* my_ptr;
} constants;

[shader("compute")]
[numthreads(1, 1, 1)]
void main() {
    InterlockedAdd(constants.my_ptr.value, 5);
}"#,
    );

    unsafe {
        println!(
            "{}",
            std::str::from_utf8_unchecked(std::slice::from_raw_parts(
                blob.get_buffer_pointer().cast(),
                blob.get_buffer_size()
            ))
        );
    }

    let (module, dblob) = session
        .load_module_from_source("example", "example.slang", &blob)
        .unwrap();
}
