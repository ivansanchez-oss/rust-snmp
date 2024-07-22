use snmp::async_session::AsyncTrapSession;

#[tokio::main]
async fn main() {
    let mut session = match AsyncTrapSession::new("127.0.0.1:162").await {
        Ok(session) => session,
        Err(err) => {
            eprintln!("{err:?}");
            return;
        }
    };

    loop {
        match session.recv_trap().await {
            Ok(message) => {
                println!("{:?}", message);
            }

            Err(err) => {
                eprintln!("{:?}", err);
            }
        }
    }
}
