# Why your async Rust is slow

This directory contains the code for the [livestream] that Tim
hosted in May 2025.

The video is a discussion of async Rust and the complexities 
that beginners face.

Some of the themes that we touch on:

- accidentially calling blocking functions
- .. which is partially explained by an incomplete understanding of async

Many people come to Rust to build web backends with the promise of performance without fear. And yet they're sometimes confused about why Rust is unable to provide as much speed as they might expect.

The video provides three cases where this occurs: printing, file I/O and locking.

For printing, there are two issues. Both are hidden. First, the `println!` macro and its siblings acquire a lock when they write. Secondly, they use blocking write syscalls. Both of those are potential issues for performance for services that are under heavy contention.

File I/O is slightly more subtle. Tokio generally recommends that developers use blocking I/O to read files, although it also offers the ability to use async also. 

Locks are sort of the same problem. In the general case, you should use the locks from the standard library rather than those provided by Tokio.

It's not trivial to know exactly what to do in all cases.

They all have a fairly common cause: you need to understand the concepts (async/non-blocking paradigm) and to read the documentation to get the best performance out of the system.

It's easy to say that newcomers are "holding it wrong". But that doesn't take awy from the fact that Rust doesn't make async easy to wield.

Compounded by the fact that Rust's model is different than Python or JavaScript. The Rust community relies on a shared trait, `std::future::Future`, but because traits are not types, the types used by different runtimes are not compatible.

In Rust, we don't just need to ask which colour our functions are, but which shade of the async colour.

The general principle of "use Tokio's types when using Tokio, rather than the standard library" also has some significant exceptions. Notably the documentation for `tokio::sync::Mutex` notes that people
should probably use `std::sync::Mutex` unless that's not satisfactory.

In the video, one of the commenters also asks about `RwLock`. After investigation, it's somewhat unclear whether that applies also.

This makes the situation somewhat more complicated for beginners. 
They don't have enough knowledge to apply nuanced decision making.




- complex interactions with the standard library's types and the types . 

## Other links

Follow Tim's YouTube Channel

[livestream]: https://youtube.com/live/wLQnfJSGJPY