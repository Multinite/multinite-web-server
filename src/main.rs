
use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};

mod util;

fn main() -> io::Result<()> {
    // Bind the server to a socket address
    let listener = TcpListener::bind("127.0.0.1:2828")?;

    // Listen for incoming connections
    for stream in listener.incoming() {
        match stream {
            // Handle each incoming connection
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr()?);
                handle_client(stream)?;
            }
            // Handle errors
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
    Ok(())
}

fn handle_client(mut stream: TcpStream) -> io::Result<()> {

    // let mut buffer = [0; 1024];
    // let bytes_read = stream.read(&mut buffer)?;
    // let request = util::decompress_string(&buffer[..bytes_read]);
    // println!("Received request: {}", request);

    // Open a file or any other data source
    let mut file = File::open("smaller_file.txt")?;

    let mut encoder = GzEncoder::new(&stream, Compression::default());

    io::copy(&mut file, &mut encoder)?;

    // Flush the encoder to ensure all data is compressed
    encoder.finish()?;

    // Buffer to hold chunks of data
    println!("Response sent");

    Ok(())
}
// let content = &mws_compiler::parse_line("let x:  = 0;");
