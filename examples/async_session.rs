use snmp::async_session::AsyncSession;
use std::time::Duration;
use tokio::time::interval;

#[tokio::main]
async fn main() {
    let session =
        AsyncSession::new("127.0.0.1:1161", b"public", Duration::from_millis(500), 0).await;

    let mut session = match session {
        Ok(session) => session,
        Err(err) => {
            eprintln!("{err:?}");
            return;
        }
    };

    let mut interval = interval(Duration::from_millis(1000));

    loop {
        match session.get(&[1, 3, 6, 1, 2, 1, 1, 5, 0]).await {
            Ok(pdu) => {
                println!("{:?}", pdu);
            }

            Err(err) => {
                eprintln!("{:?}", err);
            }
        }

        interval.tick().await;
    }
}
