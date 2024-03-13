# Billion Rows Challenge

Demonstrations of the [Billion Rows Challenge] in Rust. Hopefully at some stage
this work will end up in the [Show and Tell] section of the [official code
repository].

[![Watch live stream recording](https://img.youtube.com/vi/-1VGwmFKKf8/hqdefault.jpg)](https://www.youtube.com/embed/-1VGwmFKKf8)

## Repository structure

- `/original`: code from the [official code repository]
- `/billions`: code that was used in the [YouTube tutorial]

## Setup 

**Create the measurements**

Run `original/src/main/python/create_measurements.py` to generate 
the simulated data file. It'll appear as `original/data/measurements.txt`. 
then copy the resulting 15GB file into `billions`.


(TODO: rewrite this in Rust)

## Prior Art

Some Rust implementations to review:

- [github.com/tumdum/1brc](https://github.com/tumdum/1brc/blob/main/src/main.rs)

## Discussion of crates that are being used and how they might help

Looking at the manifests from these projects, it's somewhat interesting to see
look for patterns about where the community goes to for performance and memory efficiency.

- [github.com/tumdum/1brc](https://github.com/tumdum/1brc/blob/main/src/main.rs)

[bstr](https://crates.io/crates/bstr) (tumdum)

"binary str". Like `&str`, but not required to be UTF-8.

[fast-float](https://crates.io/crates/bstr) (tumdum)

Parsing floats efficiently is surprisingly difficult. This must be faster than
the standard library's parser, but I have not looked into it.

Interestingly, this library hasn't been updated in 2 years. Maybe it's finished?
Maybe it's missing something?

[memchr](https://crates.io/crates/memchr) (tumdum)

Fast string searching. Routines are accelerated with AVX2 vector instructions by
default.

[memmap](https://crates.io/crates/memmap)

Avoid reading everything into RAM.

[rayon](https://crates.io/crates/rayon)

Data parallelism.

[rustc-hash](https://crates.io/crates/rustc-hash)

The data source is safe, so it doesn't need DoS protection. `rustc-hash` is
faster for data that looks like tokens from a programming language, such as city names.

[smol_str](https://crates.io/crates/smol_str)

Reduces memory and increases cache coherence. The standard library's String
takes up 3*usize + data on the heap. This means 24 bytes are placed on the
stackon 64-bit machines. `smol_str` uses some of that on-stack data for the text itself.


[Billion Rows Challenge]: https://www.morling.dev/blog/one-billion-row-challenge/
[Show and Tell]: https://github.com/gunnarmorling/1brc/discussions/categories/show-and-tell
[official code repository]: https://github.com/gunnarmorling/1brc
