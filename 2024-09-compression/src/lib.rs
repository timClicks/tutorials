use std::io;

pub trait Compression {
	fn compress(&mut self, data: &[u8]) -> io::Result<Vec<u8>>;

	fn decompress(&mut self, data: &[u8]) -> io::Result<Vec<u8>>;
}
