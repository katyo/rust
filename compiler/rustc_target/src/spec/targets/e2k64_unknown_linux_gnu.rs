// ignore-tidy-linelength
use crate::spec::{base, LinkerFlavor, Target};

pub fn target() -> Target {
    let mut base = base::linux_gnu_base::opts();
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
        llvm_target: "e2k64-unknown-linux-gnu".into(),
        metadata: crate::spec::TargetMetadata {
            description: Some("64-bit Linux (kernel 3.2+, glibc 2.17+)".into()),
            tier: Some(3),
            host_tools: Some(true),
            std: Some(true),
        },
        pointer_width: 64,
        data_layout:
            "e-p:64:64:64-i1:8:8-i8:8:8-i16:16:16-i32:32:32-i64:64:64-f32:32:32-f64:64:64-f80:128:128-f128:128:128-v128:128:128-n32:64".into(),
        arch: "e2k64".into(),
        options: base,
    }
}
