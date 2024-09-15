use std::io::{self, BufReader};
use compression::Compression;

pub struct Snappy {}

impl Compression for Snappy {
    fn compress(&mut self, data: &[u8]) -> std::io::Result<Vec<u8>> {
        let buffer = Vec::with_capacity(data.len() >> 3);
        let mut data = BufReader::new(data);
        let mut encoder = snap::write::FrameEncoder::new(buffer);

        io::copy(&mut data, &mut encoder)?;

        encoder.into_inner()
            .map_err(|err| err.into_error())
    }

    fn decompress(&mut self, data: &[u8]) -> std::io::Result<Vec<u8>> {
        let mut buffer = Vec::with_capacity(data.len());
        let mut decoder = snap::read::FrameDecoder::new(data);

        io::copy(&mut decoder, &mut buffer)?;

        Ok(buffer)
    }
}


fn main() {
	let data = b"Hello, Internet!";
	let mut compressor = Snappy { };
	let dat = compressor.compress(data).unwrap(); // dat is short data
	let data2 = compressor.decompress(&dat).unwrap();

	assert_eq!(data as &[u8], &data2);
}
