# intro to axum

This is the code for my video, [A tour of the Axum web framework (while explaining a few error messages)](https://www.youtube.com/watch?v=fbBZM7cReBc).
To run it, you'll need Rust (and cargo) installed.

> *Tip:*  
> 
> Visit https://rustup.rs if you need to install Rust.

## usage

Open up two console windows. In the first, compile and run the code:

```console
$ cd intro-to-axum
$ cargo run
    Updating crates.io index
...
2023-07-04T23:47:05.335089Z  INFO intro_to_axum: connecting addr=0.0.0.0:3000
```

You now have a running web server, listening on port 3000.

In the second console, connect to the server with a web client.
I'm using `curl` here:

```console
$ curl localhost:3000/
<h1>Hello, Internet!</h1>
```