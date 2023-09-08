extern crate libc;

#[link(name = "main")] 
extern {
    fn square(z: Option<&mut libc::c_int>) -> Option<&mut libc::c_int>;
}

fn main() {
    let result;
    unsafe {
        result = square(None);
    }
    println!("Result from Zig: {:?}\n", result);

    let result2: Option<&mut i32>;
    let mut val = 420;
    unsafe {
        result2 = square(Some(&mut val));
    }
    println!("Result from Zig: {:?}\n", result2);
}