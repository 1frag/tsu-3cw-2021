extern crate libc;
use std::env;

extern {
    fn next_prime(input: libc::c_longlong) -> libc::c_longlong;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = args
        .get(1)
        .expect("Incorrect usage")
        .parse()
        .expect("Not a number");
    let output = unsafe { next_prime(input) };
    println!("next prime after {} is {}", input, output);
}
