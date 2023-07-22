mod mancala;

use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::str::from_utf8;
use std::sync::{Arc, RwLock};
use std::thread;

use mancala::Board;

enum State<'a> {
    Waiting(Vec<String>),
    Playing(Board<'a>),
}

fn handle_client(state: Arc<RwLock<State>>, mut stream: TcpStream) {
    let mut data = [0 as u8; 256];
    loop {
        data.fill(0);
        match stream.read(&mut data) {
            Ok(0) => {
                println!("Client disconnected");
                break;
            }
            Ok(size) => {
                println!("{}", from_utf8(&data).unwrap());
                // echo
                stream.write(&data[..size]).unwrap();
            }
            Err(_) => {
                println!(
                    "An error occurred, terminating connection with {}",
                    stream.peer_addr().unwrap()
                );
                stream.shutdown(Shutdown::Both).unwrap();
                break;
            }
        }
    }
}

fn main() {
    let state = Arc::new(RwLock::new(State::Waiting(Vec::new())));
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 3333");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                let shared_state = Arc::clone(&state);
                thread::spawn(move || {
                    // connection succeeded
                    handle_client(shared_state, stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
    // close the socket server
    drop(listener);
}
