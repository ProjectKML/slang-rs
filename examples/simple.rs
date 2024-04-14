use slang::GlobalSession;

fn main() {
    let global_session = GlobalSession::new().unwrap();
    let session = global_session.create_session().unwrap();
}
