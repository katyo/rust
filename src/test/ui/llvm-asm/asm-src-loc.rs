// build-fail
// dont-check-compiler-stderr
// ignore-emscripten
// ignore-e2k64

#![feature(llvm_asm)]

fn main() {
    unsafe {
        llvm_asm!("nowayisthisavalidinstruction"); //~ ERROR instruction
    }
}
