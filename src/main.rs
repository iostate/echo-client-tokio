use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

const ECHO_SERVER_ADDRESS: &str = "localhost:8000";
#[tokio::main]
async fn main() {
    // connection
    println!("connecting to {}", ECHO_SERVER_ADDRESS);
    if let Ok(mut stream) = TcpStream::connect(ECHO_SERVER_ADDRESS).await {
        println!(
            "connected to echo server {}:{} ... ",
            stream.local_addr().unwrap().ip(),
            stream.local_addr().unwrap().port()
        );

        // write hello  world
        let message = "Hello world";
        // let _ = stream.write_all(message.as_bytes()).await;
        let _ = stream.write_all(message.as_bytes()).await;
        println!("sent: {}", message);

        // accept user input
        // let mut line = String::new();
        // println!("Enter a message: ");
        // let b1 = std::io::stdin().read_line(&mut line).unwrap();
        // let _ = stream.write(line.as_bytes());

        // read the result
        let mut buffer = [0; 1024];
        // let len = stream.read(&mut buffer).await.unwrap();
        let len = stream.read(&mut buffer).await.unwrap();
        let message = String::from_utf8_lossy(&buffer);
        println!("received: {}", message);
    } else {
        println!("failed to connect to echo server {}", ECHO_SERVER_ADDRESS);
    }
}
