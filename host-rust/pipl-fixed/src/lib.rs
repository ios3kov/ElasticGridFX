//! Windows-only compatibility shim for the published pipl 0.1.1 crate.
//!
//! It preserves the existing PiPL data builder and changes only Windows resource
//! emission to the binary-file RC form fixed upstream in virtualritz/after-effects
//! commit 83dcc93734fd5db1335b6ec83cba7a6505a39dcc.

pub use pipl_upstream::{OutFlags, OutFlags2, PIPLType, Property, Stage};

pub fn plugin_build(properties: Vec<Property>) {
    let mut any_entrypoint_emitted = false;

    for prop in properties.iter() {
        match prop {
            Property::Kind(x) => {
                println!("cargo:rustc-env=PIPL_KIND={}", u32::from_le_bytes(x.as_bytes()));
            }
            Property::Name(x) => println!("cargo:rustc-env=PIPL_NAME={x}"),
            Property::Category(x) => println!("cargo:rustc-env=PIPL_CATEGORY={x}"),
            Property::AE_Effect_Match_Name(x) => {
                println!("cargo:rustc-env=PIPL_MATCH_NAME={x}");
            }
            Property::AE_Effect_Support_URL(x) => {
                println!("cargo:rustc-env=PIPL_SUPPORT_URL={x}");
            }
            Property::CodeWin64X86(x) => {
                any_entrypoint_emitted = true;
                println!("cargo:rustc-env=PIPL_ENTRYPOINT={x}");
            }
            Property::CodeMacIntel64(x) | Property::CodeMacARM64(x) => {
                any_entrypoint_emitted = true;
                println!("cargo:rustc-env=PIPL_ENTRYPOINT={x}");
            }
            Property::AE_Effect_Spec_Version { major, minor } => {
                println!("cargo:rustc-env=PIPL_AE_SPEC_VER_MAJOR={major}");
                println!("cargo:rustc-env=PIPL_AE_SPEC_VER_MINOR={minor}");
            }
            Property::AE_Reserved_Info(x) => {
                println!("cargo:rustc-env=PIPL_AE_RESERVED={x}");
            }
            Property::AE_Effect_Version {
                version,
                subversion,
                bugversion,
                stage,
                build,
            } => {
                println!(
                    "cargo:rustc-env=PIPL_VERSION={}",
                    pipl_upstream::pf_version(*version, *subversion, *bugversion, *stage, *build)
                );
            }
            Property::AE_Effect_Global_OutFlags(x) => {
                if x.contains(OutFlags::IDoDialog) {
                    println!("cargo:rustc-cfg=does_dialog");
                }
                if x.contains(OutFlags::IUseAudio)
                    || x.contains(OutFlags::AudioEffectToo)
                    || x.contains(OutFlags::AudioEffectOnly)
                {
                    println!("cargo:rustc-cfg=uses_audio");
                }
                if x.contains(OutFlags::SendUpdateParamsUI) {
                    println!("cargo:rustc-cfg=sends_update_params_ui");
                }
                println!("cargo:rustc-env=PIPL_OUTFLAGS={}", x.bits());
            }
            Property::AE_Effect_Global_OutFlags_2(x) => {
                if x.contains(OutFlags2::SupportsGpuRenderF32) {
                    println!("cargo:rustc-cfg=gpu_render");
                }
                if x.contains(OutFlags2::SupportsSmartRender) {
                    println!("cargo:rustc-cfg=smart_render");
                }
                if x.contains(OutFlags2::SupportsThreadedRendering) {
                    println!("cargo:rustc-cfg=threaded_rendering");
                }
                println!("cargo:rustc-env=PIPL_OUTFLAGS2={}", x.bits());
            }
            _ => {}
        }
    }

    if !any_entrypoint_emitted {
        println!("cargo:rustc-env=PIPL_ENTRYPOINT=EffectMain");
    }

    let pipl = pipl_upstream::build_pipl(properties).expect("build PiPL");
    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR"));
    let pipl_path = out_dir.join("pipl.bin");
    std::fs::write(&pipl_path, pipl).expect("write PiPL binary");

    let mut resource = winres::WindowsResource::new();
    resource.append_rc_content("16000 PiPL DISCARDABLE \"pipl.bin\"");
    resource.compile().expect("compile Windows PiPL resource");
}
