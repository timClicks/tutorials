use std::io;

use compression::Compression;

struct Dummy;

impl Compression for Dummy {
	fn compress(&mut self, data: &[u8]) -> std::io::Result<Vec<u8>> {
		let mut reader = io::BufReader::new(data);
		let mut buffer = Vec::with_capacity(data.len());
		io::copy(&mut reader, &mut buffer)?;

		Ok(buffer)
	}

	fn decompress(&mut self, data: &[u8]) -> std::io::Result<Vec<u8>> {
		Ok(data.to_owned())
	}
}


fn main() {
	let data = b"Hello, Internet!";
	let mut compressor = Dummy { };
	let dat = compressor.compress(data).unwrap(); // dat is short data
	let data2 = compressor.decompress(&dat).unwrap();

	assert_eq!(data as &[u8], &data2);
}
