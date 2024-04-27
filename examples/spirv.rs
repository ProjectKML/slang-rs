use std::{fs, mem};

use slang::{CompileTarget, GlobalSession, SessionDesc, SourceLanguage, TargetDesc, TargetFlags};

fn main() {
    //TODO: we need to investigate if we want to remove unsafe from the api where possible, so I'll make a big unsafe block for now
    unsafe {
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
        let session = global_session.create_session(&session_desc).unwrap();
        let mut compile_request = global_session.create_compile_request().unwrap();

        compile_request
            .process_command_line_arguments(&["-O3"])
            .unwrap();

        let translation_unit =
            compile_request.add_translation_unit(SourceLanguage::SLANG, "example");
        compile_request.add_translation_unit_source_string(
            translation_unit,
            "example.slang",
            r#"
struct MyValue {
    uint value;
}

[[vk::push_constant]] struct PushConstants {
    MyValue* my_ptr;
} constants;

[shader("compute")]
[outputtopology("triangle")]
[numthreads(1, 1, 1)]
void main() {
    InterlockedAdd(constants.my_ptr.value, 5);
}"#,
        );

        compile_request.compile().unwrap();

        let mut module = compile_request.get_module(translation_unit as _).unwrap();
        let count = module.get_defined_entry_point_count();
        println!("{}", count);

        /*
        let code = compile_request.get_entry_point_code(0);
        fs::write("test.spv", code).unwrap();
        */
        //TODO: we need to implement blob
    }
}
