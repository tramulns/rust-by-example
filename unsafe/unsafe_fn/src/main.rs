use std::slice;

fn main() {
    let some_vector = vec![1, 2, 3, 4];

    let pointer: *const u32 = some_vector.as_ptr();
    let length: usize = some_vector.len();

    unsafe {
        let my_slice: &[u32] = slice::from_raw_parts(pointer, length);

        assert_eq!(some_vector.as_slice(), my_slice);
    }
}
