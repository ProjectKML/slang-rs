use slang::{GlobalSession, SessionDesc};

fn main() {
    let global_session = GlobalSession::new().unwrap();

    let session_desc = SessionDesc {
        ..Default::default()
    };
    let session = unsafe { global_session.create_session(&session_desc) }.unwrap();
    let compile_request = unsafe { global_session.create_compile_request() }.unwrap();
}
