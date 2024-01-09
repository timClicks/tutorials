# Create a spectrogram generator in Rust

Source code for [this video](https://youtube.com/live/PoaHybRUC18).

The code uses a single `main()` function, but is conceptually divided into three areas:

- **I/O:**  
  reading in a WAV file (uncompressed audio) and writing a PNG. We handle this with
  external libraries. Specifically the `hound` and `image` crates.  
- **FFT:**  
  taking multiple windows of audio samples, then applying an Fourier transormation
  at each window. This translates the 1D audio stream into a 2D histogram.  
- **Creating the image:**  
  The histogram data is then translated to a colored pixels.

Perhaps unfortunately, the last two steps appear together within the code. Each time
step and pixel is processed independently.
