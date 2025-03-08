use std::net::SocketAddr;

use tokio::{io::AsyncWriteExt, net::TcpListener};
use tracing::{info, warn};

const BUF_SIZE: usize = 4096;

async fn process_redis_conn(
    mut stream: tokio::net::TcpStream,
    addr: SocketAddr,
) -> anyhow::Result<()> {
    loop {
        stream.readable().await?;

        let mut buf = Vec::with_capacity(BUF_SIZE);

        match stream.try_read_buf(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                let line = String::from_utf8_lossy(&buf);
                info!("read {} bytes: {:?}", n, line);
                stream.write_all(b"+OK\r\n").await?;
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => return Err(e.into()),
        }
    }

    warn!("dredis: Connection {} closed", addr);

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    // build a listener
    let addr = "0.0.0.0:6379";
    info!("Dummy redis: listening on {}", addr);
    let listener = TcpListener::bind(addr).await?;

    loop {
        let (stream, raddr) = listener.accept().await?;
        info!("Accepted connection from {}", raddr);

        tokio::spawn(async move {
            if let Err(e) = process_redis_conn(stream, raddr).await {
                warn!("Error processing connection with {}: {:?}", raddr, e);
            }
        });
    }
}
