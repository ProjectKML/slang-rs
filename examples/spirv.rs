use std::{mem, ops::Deref};

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

    let mut module = session
        .load_module_from_source("example", "example.slang", &blob, None)
        .unwrap();

    let entry_point = module.find_entry_point_by_name("main").unwrap();

    let component_type = session
        .create_composite_component_type(
            &[module.deref().clone(), entry_point.deref().clone()],
            None,
        )
        .unwrap();
    let code = component_type.get_entry_point_code(0, 0, None).unwrap();
}
