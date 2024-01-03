#[cxx::bridge]
mod ffi {
    #[repr(u32)]
    enum Variant {
        #[cfg(not(feature = "v2"))]
        A,
        B,
    }

    unsafe extern "C++" {
        include!("cxx-not-feature/header.h");

        type Variant;
    }
}

fn main() {
    println!("Hello, world!");
}
