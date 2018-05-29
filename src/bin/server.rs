use std::fmt;

pub fn p<T>(x: T) -> T
where
    T: fmt::Debug,
{
    println!("{:?}", x);
    x
}
use std::thread;

extern crate websocket;
use std::sync::mpsc;
use websocket::OwnedMessage;
use websocket::sync::Server;

pub fn start_websocket_server() {
    let server = Server::bind("127.0.0.1:1234").unwrap();
    let mut next_id = 0;
    let mut generate_id = || {
        next_id += 1;
        next_id
    };

    for request in server.filter_map(Result::ok) {
        let id = generate_id();

        // Spawn a new thread for each connection.
        thread::spawn(move || {
            let client = request.accept().unwrap();

            println!("Client connected: {}", id);

            let (mut receiver, mut sender) = client.split().unwrap();

            let (ws_tx, ws_rx) = mpsc::channel();

            thread::spawn(move || {
                for ws_msg in ws_rx.iter() {
                    sender.send_message(&ws_msg).ok();
                }
            });

            for message in receiver.incoming_messages() {
                match message {
                    Ok(OwnedMessage::Text(msg)) => println!("Got msg from {:?}: {:?}", id, msg),

                    Ok(OwnedMessage::Close(_)) => {
                        println!("Remove tabs from connection {}", id);
                        ws_tx.send(OwnedMessage::Close(None)).unwrap();
                        return;
                    }

                    Err(e) => {
                        println!("Error {:?}, closing connection", e);
                        ws_tx.send(OwnedMessage::Close(None)).unwrap();
                        return;
                    }

                    _ => {}
                }
            }
        });
    }
}

fn main() {
    start_websocket_server();
}
