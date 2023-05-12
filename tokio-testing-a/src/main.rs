use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8888").await.unwrap();
    println!("Listening on http://{}", listener.local_addr().unwrap());

    loop {
        let (mut socket, _) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            let mut buffer = [0; 1024];

            loop {
                let nbytes = match socket.read(&mut buffer).await {
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(_) => return,
                };

                let response = "HTTP/1.1 200 OK\r\nConnection: keep-alive\r\nContent-Length: 13\r\n\r\nHello, world!";
                socket.write_all(response.as_bytes()).await.unwrap();

                if !buffer[nbytes - 4..nbytes].eq(b"\r\n\r\n") {
                    return;
                }
            }
        });
    }
}
