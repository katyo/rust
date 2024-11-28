// ignore-tidy-linelength
use crate::spec::{base, LinkerFlavor, Target, Cc, Lld};

pub fn target() -> Target {
    let mut base = base::linux_gnu::opts();
    base.max_atomic_width = Some(64);
    base.position_independent_executables = false;
    base.add_pre_link_args(LinkerFlavor::Gnu(Cc::Yes, Lld::No), &["-fPIC"]);
    base.post_link_args.insert(
        LinkerFlavor::Gnu(Cc::Yes, Lld::No),
        vec![
            "-llccrt_s".into(),
            "-llcc".into(),
            "-lm".into(),
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
            "e-m:e-p:64:64-i64:64:64-f80:128:128-n32:64-S128".into(),
        arch: "e2k64".into(),
        options: base,
    }
}
