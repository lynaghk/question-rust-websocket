extern crate websocket;

use std::thread;
use websocket::OwnedMessage;
use websocket::client::ClientBuilder;

fn connect(msg: String) -> websocket::sync::Client<std::net::TcpStream> {
    let mut client = ClientBuilder::new("ws://127.0.0.1:1234")
        .unwrap()
        .connect_insecure()
        .unwrap();

    client.send_message(&OwnedMessage::Text(msg)).unwrap();

    return client;
}

fn main() {
    let _a = connect("A".to_string());
    thread::sleep_ms(10000);
}
