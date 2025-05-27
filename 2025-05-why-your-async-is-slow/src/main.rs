// WHY IS ASYNC RUST SO HARD?

// 1.1 async does things differently
//
// 1.2 Rust's implmentation of async is also different 

// 2.1 people arrive at Rust and the first thing they want
//    to do is write a web API (from Python and JS/TS). 
//
// 2.2 All of Rust's web frameworks seem to require async.

use tokio::{self, io::AsyncWriteExt};
use tokio::io::AsyncWrite;
use std::{fmt::write, io::{stdin, stdout, Write}};

#[allow(unused)]
async fn issue_println() -> i32 {
    // println!("Hello, world"); // help! accidential blocking I/O
    
    // PROBLEM CODE: standard libary blocks
    //
    // let mut out = stdout();
    // let mut out = out.lock();
    // write!(out, "Hello, world!");
     
    // "CORRECT" CODE: non-blocking
    let mut out = tokio::io::stdout();
    out.write(b"Hello, world!");

    123
}

#[allow(unused)]
async fn issue_files() -> i32 {
    // std::io::File (std vs tokio)
    // CORRECT Code uses tokio::fs::* and tokio::io::*
    todo!()
}

#[allow(unused)]
async fn issue_locks() -> i32 {
    // Mutex (std vs tokio)
    todo!()
}

fn main() {
    let output = work();
    println!("Hello, Internet!");
}
