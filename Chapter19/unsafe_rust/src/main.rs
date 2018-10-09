use std::slice;

extern "C" {
    fn abs(input: i32) -> i32;
}

static mut COUNTER: u32 = 0;

fn main() {
    simple_unsafe();
    advanced_unsafe();
    external_unsafe();
    static_unsafe();

}

fn static_unsafe() {
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn external_unsafe() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

fn advanced_unsafe() {
    let mut v = vec![1,2,3,4,5,6];
    let r = &mut v[..];

    let (a, b) = split_at_mut(r, 3); //r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]); 
}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid)
        )
    }
}

fn simple_unsafe() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    println!("Raw Address One: {:?}", r1);
    println!("Raw Address Two: {:?}", r2);

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}
