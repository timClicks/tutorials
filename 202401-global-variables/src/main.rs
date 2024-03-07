// static mut SEED: u64 = 12345412934873291213;

// /// an implementation of the XORShift
// fn rand() -> u64 {
//     unsafe {
//         SEED <<= 13;
//         SEED >>= 17;
//         SEED <<= 5;
//         SEED
//     }
// }

// use std::sync::atomic::{AtomicU64, Ordering};
// static SEED: AtomicU64 = AtomicU64::new(12345412934873291213);

// /// an implementation of the XORShift
// fn rand() -> u64 {
//     let mut seed = SEED.load(Ordering::SeqCst);
//     seed <<= 13;
//     seed >>= 17;
//     seed <<= 5;
//     SEED.store(seed, Ordering::SeqCst);
//     seed
// }

// use std::sync::{Arc, Mutex};
// use once_cell::sync::Lazy;

// static mut SEED: Lazy<Arc<Mutex<u64>>> = Lazy::new(|| {Arc::new(Mutex::new(12345412934873291213))});

// /// an implementation of the XORShift
// fn rand() -> u64 {
//     unsafe {
//         let seed_mutex = SEED.clone();
//         let mut seed_guard = seed_mutex
//             .lock()
//             .unwrap();

//         let mut seed = *seed_guard;
//         seed <<= 13;
//         seed >>= 17;
//         seed <<= 5;
//         *seed_guard = seed;

//         seed
//     }
// }

use std::cell::Cell;

thread_local! {
    static SEED: Cell<u64> = Cell::new(12345412934873291213);
}

/// an implementation of the XORShift
fn rand() -> u64 {
    let mut seed = SEED.get();

    seed <<= 13;
    seed >>= 17;
    seed <<= 5;
    SEED.set(seed);
    seed
}


fn main() {
    println!("{}", rand());
    println!("{}", rand());
    println!("{}", rand());
}
