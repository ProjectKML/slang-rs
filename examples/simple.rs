use slang::{GlobalSession, SessionDesc};

fn main() {
    let global_session = GlobalSession::new().unwrap();

    let session_desc = SessionDesc {
        ..Default::default()
    };
    let session = global_session.create_session(&session_desc).unwrap();
}
