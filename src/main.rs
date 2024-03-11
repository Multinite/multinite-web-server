
use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};

mod util;

fn main() -> io::Result<()> {

    let mut file = File::open("smaller_file.txt")?;

    // Bind the server to a socket address
    let listener = TcpListener::bind("127.0.0.1:2828")?;

    // Listen for incoming connections
    for stream in listener.incoming() {
        match stream {
            // Handle each incoming connection
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr()?);
                handle_client(stream, file)?;
            }
            // Handle errors
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
    Ok(())
}

fn handle_client(stream: TcpStream, mut file: File) -> io::Result<()> {

    let mut encoder = GzEncoder::new(&stream, Compression::default());

    io::copy(&mut file, &mut encoder)?;

    encoder.finish()?;

    println!("Response sent");

    Ok(())
}
