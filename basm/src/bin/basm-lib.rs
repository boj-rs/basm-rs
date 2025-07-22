#![cfg_attr(not(test), no_builtins)]
#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(rustfmt, rustfmt_skip)] // temporary fix to keep compiler_builtins at the top to avoid linker errors

// "nintendo" indicates the x86_64-unknown-linux-gnu-short target
// (picked random word to avoid rustc warning)
#[cfg(not(target_vendor = "nintendo"))]
extern crate compiler_builtins;
extern crate alloc;
extern crate basm_std as basm;
mod lang_items;

#[cfg_attr(test, allow(dead_code))]
#[path = "../solution.rs"]
mod solution;

#[cfg(test)]
mod verify_test_works {
    fn add(x: i64, y: i64) -> i64 {
        x + y
    }

    #[test]
    fn run() {
        assert_eq!(8, add(5, 3));
    }
}
