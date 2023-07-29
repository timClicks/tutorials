fn main() {
    let mut p = Box::new(10);
    drop(p);
    *p = 20; // error[E0382]: borrow of moved value: `p`
}
