#![allow(dead_code)]

// Ownership
// what: responsible for cleaning up at the scope (*)
// what: invokes the Drop implementation for T when needed
// why: guard against clean up occurring twice
// note: Copy can be used to opt-out of ownership
// note: references can be used to opt-out of ownership
// passing ownership: rebind with let
// passing ownership: via a function (fn) or a closure (anon struct)
// passing ownership: via a return value

// References / Borrowing
// what: two terms are (almost) the same
// what: shared reference, a.k.a immutable borrow or read-only borrow
// what: unique reference, a.k.a mutable borrow or read/write borrow

// Lifetimes
// what: span of execution time where a variable is valid to access

// Lifetime parameters
// what: binding two types together, or at least distinguish between
//       'static and some non-static Lifetime

fn read<'a, 'b: 'a>(b: Option<&'a Book>, s: &'b Shelf) -> &'a Book {
    let book = b.unwrap_or_else(|| { &s.books[0] });

    println!("{book:?}");

    book
}


#[derive(Debug)]
struct Book {
    pages: u32,
}

#[derive(Debug)]
struct Shelf {
    books: Vec<Book>,
}

// todo: inspect std::mem::drop
fn inspect<T: std::fmt::Debug>(data: &T) {
    println!("inspection: {data:?}");
}

fn main() {
    let b1 = Book { pages: 450 };
    let b2 = Book { pages: 540 };
    let s = Shelf { books: vec![b1, b2] };

    let b3 = Book { pages: 54 };

    let work = || {
        inspect(&s)
    };
    work();

    println!("{s:?}");

    std::mem::drop(b3);

    read(b3, s);

    // println!("b1: {b1:?}!");
    // b1 ends its scope
}