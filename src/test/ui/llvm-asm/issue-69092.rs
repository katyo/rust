// build-fail
// ignore-emscripten no asm! support
// Regression test for #69092
// ignore-e2k64

#![feature(llvm_asm)]

fn main() {
    unsafe { llvm_asm!(".ascii \"Xen\0\""); }
    //~^ ERROR: expected string in '.ascii' directive
}
