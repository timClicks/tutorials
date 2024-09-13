use std::io::{Read, Write};
use flate2::{read, write};

use compression::Compression;



struct Deflate;

impl Compression for Deflate {
    fn compress(&mut self, data: &[u8]) -> std::io::Result<Vec<u8>> {
        let buffer = Vec::with_capacity(data.len() >> 3);
        let mut enc = write::DeflateEncoder::new(buffer, flate2::Compression::best());
        enc.write_all(data)?;

        enc.finish()
    }

    fn decompress(&mut self, data: &[u8]) -> std::io::Result<Vec<u8>> {
        let mut buffer = Vec::with_capacity(data.len());
        let mut dec = read::DeflateDecoder::new(data);
        dec.read_to_end(&mut buffer)?;

        Ok(buffer)
    }
}


fn main() {
	let data = b"Hello, Internet! Be wary of small strings";
	let mut compressor = Deflate { };
	let dat = compressor.compress(data).unwrap(); // dat is short data
	let data2 = compressor.decompress(&dat).unwrap();

	assert_eq!(data as &[u8], &data2);
}