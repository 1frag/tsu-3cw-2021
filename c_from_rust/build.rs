extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/next_prime.c")
        .compile("libnext_prime.a");
}
