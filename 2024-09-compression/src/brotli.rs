//! Brötli (pronounced brute-lee) is a Swiss German word for a small bread roll.
//! It uses an encoding of its compressed form that makes it very fast to decompress.

use brotli::enc::backward_references::BrotliEncoderMode;
use brotli::CompressorReader;
use brotli::Decompressor;
use brotli::enc::BrotliEncoderParams;
use std::io;

use compression::Compression;

struct Brötli {
	/// Higher values produce better compression, but are slower. Corresponds to
	/// the [BROTLI_PARAM_QUALITY] parameter of the original library.
	///
	/// [BROTLI_PARAM_QUALITY]: https://www.brotli.org/encode.html#a9a8
	quality: i32,

	/// Specifies the width in bits of the encoding window. Significantly affects
	/// memory use.
	///
	/// Must be between 10
	/// and 24, inclusive. The parameter name lgwin is short for "log of bit
	/// window". It corresponds to the [BROTLI_PARAM_LGWIN] parameter of the
	/// original library.
	///
	/// [BROTLI_PARAM_LGWIN]: https://www.brotli.org/encode.html#a5a8
	lgwin: i32,

	/// The size of the internal buffer. Wider values produce better compression
	/// at the expense of higher latency.
	buffer_size: usize,
}

impl Default for Brötli {
	fn default() -> Self {
		Self {
			quality: 11,
			lgwin: 20,
			buffer_size: 0x1000
		}
	}
}

impl Brötli {
	fn new(quality: i32, lgwin: i32, buffer_size: usize) -> Self {
		assert!(quality < 12);
		assert!(lgwin >= 10);
		assert!(lgwin <= 24);
		Brötli {
			quality, lgwin, buffer_size
		}
	}
}

impl Compression for Brötli {
	fn compress(&mut self, data: &[u8]) -> io::Result<Vec<u8>> {
		let mode = match std::str::from_utf8(data) {
			Ok(_) => BrotliEncoderMode::BROTLI_MODE_TEXT,
			Err(_) => BrotliEncoderMode::BROTLI_MODE_GENERIC,
		};

		let params = BrotliEncoderParams {
			lgwin: self.lgwin,
			quality: self.quality,
			mode,
			..Default::default()
		};

		let mut comp = CompressorReader::with_params(data, self.buffer_size, &params);

        let mut compressed_data = Vec::new();
        io::copy(&mut comp, &mut compressed_data)?;
        Ok(compressed_data)
	}

	fn decompress(&mut self, data: &[u8]) -> io::Result<Vec<u8>> {
		let mut decomp = Decompressor::new(data, self.buffer_size);
	    let mut decompressed_data = Vec::new();
	    io::copy(&mut decomp, &mut decompressed_data)?;
	    Ok(decompressed_data)
	}
}

fn main() {
	let mut brötli = Brötli::new(11, 20, 16*1024);
	let data = b"Hello Internet, how are you?

	Let's squish some bytes.

	For small strings, \"compression\" algorithms can actually increase the size of the string.";

	let dat = brötli.compress(data).unwrap();
	let data2 = brötli.decompress(&dat).unwrap();
	assert_eq!(data as &[u8], data2);

	println!("original:   {}", data.len());
	println!("compressed: {}", dat.len());
}
