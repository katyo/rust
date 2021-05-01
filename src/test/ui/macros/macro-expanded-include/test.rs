// ignore-emscripten no llvm_asm! support
// ignore-e2k64
// build-pass (FIXME(62277): could be check-pass?)
#![feature(asm)]
#![allow(unused)]

#[macro_use]
mod foo;

m!();
fn f() { n!(); }


fn main() {}
