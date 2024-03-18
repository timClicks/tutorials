# Build your own HTTP framework

*Support this work by [becoming a patron].*

This repository contains the code created during the 17 March 2024 live stream.
The [recording is available at on YouTube].

[becoming a patron]: https://www.patreon.com/timClicks
[recording is available at on YouTube]: https://youtu.be/hn64haI8mOI?si=FXFZsdGyLt5x3Xct

## Repository structure

- `example.rs`: an example web server that I actually found while [investigating unikernels]
- `streamtrain/`: the partially-complete framework that was built on stream

[investigating unikernels]: https://github.com/nanovms/ops-examples/tree/5d70980ed1dbb3c42fa337f0eba4cf97f1314819/rust/02-http-hello-world

## Advice for the curious

If you're embarking a journey like this yourself, I recommend consulting a few
other crates.

- [`http`] provides a `Request` and `Response` type. It's built by the people
  who make Tokio.
- [`tiny_http`] provides its own opinions about what a `Request` and `Response`
  are, and also handles a lot of the connection handling.

[`http`]: https://crates.io/crates/http
[`tiny_http`]: https://crates.io/crates/tiny_http
