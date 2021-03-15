fn main() {
    let raw_p: *const u32 = &10;
    unsafe {
        assert!(*raw_p == 10);
    }
    unsafe {
        println!("{}", *raw_p);
    }
}
