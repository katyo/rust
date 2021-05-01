// ignore-tidy-linelength

use crate::spec::{LinkerFlavor, Target};

pub fn target() -> Target {
    let mut base = super::linux_gnu_base::opts();
    base.max_atomic_width = Some(64);
    base.position_independent_executables = false;
    base.pre_link_args
        .get_mut(&LinkerFlavor::Gcc)
        .unwrap()
        .push("-fPIC".to_string());
    base.post_link_args.insert(
        LinkerFlavor::Gcc,
        vec![
            "-llccrt_s".to_string(),
            "-llcc".to_string(),
            "-lm".to_string(),
        ],
    );

    Target {
        llvm_target: "e2k64-unknown-linux-gnu".to_string(),
        pointer_width: 64,
        arch: "e2k64".to_string(),
        data_layout: "e-p:64:64:64-i1:8:8-i8:8:8-i16:16:16-i32:32:32-i64:64:64-f32:32:32-f64:64:64-f80:128:128-f128:128:128-v128:128:128-n32:64".to_string(),

        options: base,
    }
}
