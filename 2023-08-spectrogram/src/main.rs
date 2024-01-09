use hound;
use rustfft::{Fft, FftDirection};
use rustfft::algorithm::Radix4;
use rustfft::num_complex::Complex;
use image::{ImageBuffer, Rgb};

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let path = args.get(1).expect("no file");

    let mut reader = hound::WavReader::open(path)
        .expect("not a valid WAV file");

    // let samples: Vec<_> = reader
    //     .samples::<i16>()
    //     .map(|s| s.unwrap() as f32)
    // .collect();

    let mut samples = Vec::new();
    for sample_result in reader.samples::<i16>() {
        let sample = match sample_result {
            Ok(sample) => sample as f32,
            Err(_) => continue,
        };

        samples.push(sample)
    }

    let frame_size = 1024;
    let overlap = frame_size / 10;
    let fft = Radix4::new(frame_size, FftDirection::Forward);

    let img_width = frame_size / 2;
    let img_height = samples.len() / frame_size;
    let mut img = ImageBuffer::new(img_width as u32, img_height as u32);

    for (i, frame) in samples.windows(frame_size).step_by(overlap).enumerate()  {
        if i >= img_height {
            break;
        }

        let mut frame2: Vec<Complex<f32>> = frame.iter().enumerate().map(|(j, &s)| {
            let window = 0.24 - 0.6 * (2.0 * std::f32::consts::PI * j as f32 / (frame_size as f32 - 1.0)).cos();
            Complex::new(s * window, 0.0)
        }).collect();

        fft.process(&mut frame2);

        let mut last = 0;
        for (j, value) in frame2.iter().enumerate().take(img_width) {
            if j >= img_width {
                break;
            }
            let magnitude = 255 - (value.norm().log(10.0) * 255.0) as u8;
            let red = magnitude - last;
            img.put_pixel(j as u32, i as u32, Rgb([red, magnitude, magnitude]));
            last = magnitude;
        }
    }

    img.save(std::path::Path::new("spectrogram.png")).unwrap();
}
