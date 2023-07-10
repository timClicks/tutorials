
use std::cmp::max;
use std::mem::{align_of, size_of};
use std::ptr;
use libc;

#[derive(Debug)]
struct Portal<T>(ptr::NonNull<T>);

impl<T> Portal<T> {
    fn new(value: T) -> Self {
        let mut memptr: *mut T = ptr::null_mut();

        let addr = (&mut memptr as *mut *mut T).cast();
        let alignment = max(align_of::<T>(), size_of::<usize>());
        let how_much_to_allocate = size_of::<T>();

        let err_code = unsafe {
             libc::posix_memalign(
                addr,
                alignment,
                how_much_to_allocate,
            )
        };
        match err_code {
            libc::EINVAL => panic!("alignment incorrect!"),
            libc::ENOMEM => panic!("no memory!"),
            _ => (),
        }

        let ptr = ptr::NonNull::new(memptr).unwrap();

        // !!! please don't block this thread

        unsafe {
            ptr.as_ptr().write(value);
        }

        Self(ptr)
    }
}

use std::ops::{Deref, DerefMut};

impl<T> Deref for Portal<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe {
            // Safety: The pointer is aligned, initialized, and dereferenceable
            //   by the logic in [`Self::new`]. We require readers to borrow the
            //   Carton, and the lifetime of the return value is elided to the
            //   lifetime of the input. This means the borrow checker will
            //   enforce that no one can mutate the contents of the Carton until
            //   the reference returned is dropped.
            self.0.as_ref()
        }
    }
}

impl<T> DerefMut for Portal<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            // Safety: The pointer is aligned, initialized, and dereferenceable
            //   by the logic in [`Self::new`]. We require writers to mutably
            //   borrow the Carton, and the lifetime of the return value is
            //   elided to the lifetime of the input. This means the borrow
            //   checker will enforce that no one else can access the contents
            //   of the Carton until the mutable reference returned is dropped.
            self.0.as_mut()
        }
    }
}

#[derive(Debug)]
struct Oof<T>(T);

fn main() {
    let n = Oof(123);
    let portal = Box::new(n);

    println!("{:?}", *portal);
}
