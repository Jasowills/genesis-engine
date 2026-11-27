//! Evil fixture: never returns; must be stopped by epoch interruption / timeout.
fn main() {
    let mut counter: u64 = 0;
    loop {
        counter = counter.wrapping_add(1);
        std::hint::black_box(counter);
    }
}
