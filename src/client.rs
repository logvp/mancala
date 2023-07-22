use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::str::from_utf8;

fn main() {
    match TcpStream::connect("localhost:3333") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 3333");

            let mut input = String::new();
            loop {
                input.clear();
                io::stdin().read_line(&mut input).unwrap();

                stream.write(input.as_bytes()).unwrap();
                println!("Sent {:?}, awaiting reply...", input);

                let mut reader = BufReader::new(&stream);
                let mut buffer: Vec<u8> = Vec::new();
                match reader.read_until(b'\n', &mut buffer) {
                    Ok(_) => {
                        println!("response: {}", from_utf8(&buffer).unwrap());
                        if buffer != input.as_bytes() {
                            println!("Unexpected reply: {:?}", buffer);
                            break;
                        }
                    }
                    Err(e) => {
                        println!("Failed to receive data: {e}");
                        break;
                    }
                }
            }
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}
