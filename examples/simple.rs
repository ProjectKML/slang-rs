use std::mem;

use slang::{CompileTarget, GlobalSession, SessionDesc, TargetDesc, TargetFlags};

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
}
