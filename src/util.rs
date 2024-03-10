use std::io::prelude::*;
use flate2::write::GzEncoder;
use flate2::Compression;
use flate2::read::GzDecoder;
use std::io::Cursor;

pub fn decompress_string(compressed_data: &[u8]) -> String {
    let mut decoder = GzDecoder::new(Cursor::new(compressed_data));
    let mut decompressed_data = String::new();
    decoder.read_to_string(&mut decompressed_data).unwrap();
    decompressed_data
}

pub fn compress_string(input: &str) -> Vec<u8> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(input.as_bytes()).unwrap();
    encoder.finish().unwrap()
}
