use std::net::TcpListener;

// constants
const SIROCCO_SERVER_ADDRESS: &str = "127.0.0.1:8000";
fn main() {
    // bind
    let listener = TcpListener::bind(SIROCCO_SERVER_ADDRESS);

    // start
    println!("sirocco listening {}", SIROCCO_SERVER_ADDRESS);

    for stream in listener.incoming() {
        let _stream = stream.unwrap();

        println!("connection established!");
    }
}
