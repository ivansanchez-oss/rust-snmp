use snmp::sync_session::SyncSession;
use std::{thread::sleep, time::Duration};

fn main() {
    let session = SyncSession::new(
        "127.0.0.1:1161",
        b"public",
        Some(Duration::from_millis(2000)),
        0,
    );

    let mut session = match session {
        Ok(session) => session,
        Err(err) => {
            eprintln!("{err:?}");
            return;
        }
    };

    loop {
        match session.get(&[1, 3, 6, 1, 2, 1, 1, 5, 0]) {
            Ok(pdu) => {
                println!("{:?}", pdu);
            }

            Err(err) => {
                eprintln!("{:?}", err);
            }
        }

        sleep(Duration::from_millis(1000));
    }
}
