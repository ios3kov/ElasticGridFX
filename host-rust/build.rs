#[cfg(not(target_os = "windows"))]
use pipl::*;
#[cfg(target_os = "windows")]
use pipl_fixed::*;
use std::path::PathBuf;

const PF_PLUG_IN_VERSION: u16 = 13;
const PF_PLUG_IN_SUBVERS: u16 = 28;

fn generate_metal_header(root: &std::path::Path, out_dir: &std::path::Path) {
    let shader_path = root.join("src/gpu/warp.metal");
    let source = std::fs::read_to_string(&shader_path).expect("read Metal shader");
    assert!(!source.contains(")EGMETAL\""), "Metal shader contains raw-string terminator");
    let header = format!(
        "#pragma once\n#include <cstddef>\nstatic const char kElasticGridMetalSource[] = R\"EGMETAL({})EGMETAL\";\nstatic constexpr std::size_t kElasticGridMetalSourceLength = sizeof(kElasticGridMetalSource) - 1;\n",
        source
    );
    std::fs::write(out_dir.join("elasticgrid_metal_source.h"), header).expect("write Metal shader header");
}

fn generate_macos_bundle_metadata(out_dir: &std::path::Path) {
    let target_profile_dir = out_dir.join("../../..");
    let package_name = std::env::var("CARGO_PKG_NAME").expect("CARGO_PKG_NAME");
    let pkginfo_path = target_profile_dir.join(format!("{package_name}_PkgInfo"));
    let plist_path = target_profile_dir.join(format!("{package_name}_Info.plist"));

    // pipl 0.1.1 only emits the .rsrc on macOS. Keep the dependency pinned and
    // generate the two bundle metadata files expected by the packaging step.
    std::fs::write(&pkginfo_path, b"eFKTFXTC").expect("write macOS PkgInfo");

    let plist = r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleIdentifier</key>
    <string>com.elasticgrid.fx</string>
    <key>CFBundlePackageType</key>
    <string>eFKT</string>
    <key>CFBundleSignature</key>
    <string>FXTC</string>
</dict>
</plist>
"#;
    std::fs::write(&plist_path, plist).expect("write macOS Info.plist");
}

fn main() {
    println!("cargo:rustc-check-cfg=cfg(fstr_lifecycle_probe)");
    println!("cargo:rustc-check-cfg=cfg(fstr_binding_probe)");
    println!("cargo:rustc-check-cfg=cfg(fstr_auto_binding)");
    // The verified automatic plane is part of normal builds. The old cfg names
    // remain as internal module gates so research/no-default builds can still
    // reproduce earlier baselines without renaming persistent parameter IDs.
    if std::env::var_os("CARGO_FEATURE_NATIVE_PLANE").is_some() {
        for name in ["fstr_lifecycle_probe","fstr_binding_probe","fstr_auto_binding"] {
            println!("cargo:rustc-cfg={name}");
        }
    }
    // The after-effects 0.4.0 macro expands these cfg names in the destination
    // crate. Register them explicitly so modern rustc check-cfg / Clippy can
    // validate the expansion without treating supported host cfgs as unknown.
    for cfg_name in ["does_dialog", "with_premiere", "threaded_rendering", "catch_panics"] {
        println!("cargo:rustc-check-cfg=cfg({cfg_name})");
    }

    // Never let a Rust panic cross the After Effects C ABI in release builds.
    // The after-effects host macro wraps EffectMain in catch_unwind when this cfg is set.
    println!("cargo:rustc-cfg=catch_panics");
    let root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
        .parent().unwrap().to_path_buf();
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();

    // Identity is generated only in OUT_DIR. The Python tool validates Git or a
    // hash-checked source snapshot and emits Cargo change-tracking directives.
    let python_command = if target_os == "windows" { "python" } else { "python3" };
    let identity = std::process::Command::new(python_command)
        .arg(root.join("tools/build_identity.py"))
        .arg("generate").arg("--root").arg(&root)
        .arg("--out").arg(&out_dir)
        .arg("--target").arg(std::env::var("TARGET").expect("TARGET"))
        .arg("--profile").arg(std::env::var("PROFILE").expect("PROFILE"))
        .output().expect("run source/build identity generator");
    assert!(identity.status.success(), "build identity: {}", String::from_utf8_lossy(&identity.stderr));
    print!("{}", String::from_utf8(identity.stdout).expect("UTF-8 Cargo directives"));


    let mut cpp = cc::Build::new();
    cpp.cpp(true)
        .std("c++20")
        .include(root.join("src"))
        .file(root.join("src/core/GridModel.cpp"))
        .file(root.join("src/core/GridCodec.cpp"))
        .file(root.join("src/core/WarpMath.cpp"))
        .file(root.join("src/core/CpuRenderer.cpp"))
        .file(root.join("src/core/PlaneTransform.cpp"))
        .file(root.join("src/core/PlaneWarp.cpp"))
        .file(root.join("src/core/PlaneRenderer.cpp"))
        .file(root.join("src/bridge/plane_ffi.cpp"))
        .file(root.join("src/bridge/elasticgrid_ffi.cpp"))
        .file(root.join("src/bridge/control_grid_ffi.cpp"));

    if std::env::var("PROFILE").as_deref() == Ok("release") {
        cpp.opt_level(3);
    }
    cpp.flag_if_supported("-fvisibility=hidden")
        .flag_if_supported("-fno-math-errno")
        // Keep CPU/GPU numerical behavior close while allowing hardware FMA.
        // No fast-math: NaN/Inf and IEEE semantics remain intact.
        .flag_if_supported("-ffp-contract=fast")
        .warnings(true)
        .compile("elasticgrid_core");

    if target_os == "macos" {
        generate_macos_bundle_metadata(&out_dir);
        generate_metal_header(&root, &out_dir);
        let mut metal = cc::Build::new();
        metal.cpp(true)
            .std("c++20")
            .include(root.join("src"))
            .include(&out_dir)
            .file(root.join("src/gpu/metal_backend.mm"))
            .file(root.join("src/bridge/hand_cursor.mm"))
            .flag_if_supported("-fobjc-arc")
            .flag_if_supported("-fvisibility=hidden")
            .warnings(true);
        if std::env::var("PROFILE").as_deref() == Ok("release") {
            metal.opt_level(3);
        }
        metal.compile("elasticgrid_metal");
        println!("cargo:rustc-link-lib=framework=Metal");
        println!("cargo:rustc-link-lib=framework=Foundation");
        println!("cargo:rustc-link-lib=framework=AppKit");
    }

    for path in [
        "src/core/GridModel.cpp",
        "src/core/GridModel.h",
        "src/core/GridCodec.cpp",
        "src/core/GridCodec.h",
        "src/core/WarpMath.cpp",
        "src/core/WarpMath.h",
        "src/core/CpuRenderer.cpp",
        "src/core/CpuRenderer.h",
        "src/core/PlaneTransform.cpp",
        "src/core/PlaneTransform.h",
        "src/core/PlaneWarp.cpp",
        "src/core/PlaneWarp.h",
        "src/core/PlaneRenderer.cpp",
        "src/core/PlaneRenderer.h",
        "src/bridge/plane_ffi.cpp",
        "src/bridge/plane_ffi.h",
        "src/core/SimdPixelOps.h",
        "src/bridge/elasticgrid_ffi.cpp",
        "src/bridge/elasticgrid_ffi.h",
        "src/bridge/hand_cursor.mm",
        "src/gpu/warp.metal",
        "src/gpu/metal_backend.mm",
    ] {
        println!("cargo:rerun-if-changed={}", root.join(path).display());
    }

    let mut out_flags2 =
        OutFlags2::SupportsSmartRender |
        OutFlags2::SupportsQueryDynamicFlags |
        OutFlags2::FloatColorAware |
        OutFlags2::SupportsThreadedRendering |
        OutFlags2::SupportsGetFlattenedSequenceData;
    if target_os == "macos" {
        out_flags2 |= OutFlags2::SupportsGpuRenderF32;
    }

    plugin_build(vec![
        Property::Kind(PIPLType::AEEffect),
        Property::Name("FSTR Stretch"),
        Property::Category("FSTR Effects"),

        #[cfg(target_os = "windows")]
        Property::CodeWin64X86("EffectMain"),
        #[cfg(target_os = "macos")]
        Property::CodeMacIntel64("EffectMain"),
        #[cfg(target_os = "macos")]
        Property::CodeMacARM64("EffectMain"),

        Property::AE_PiPL_Version { major: 2, minor: 0 },
        Property::AE_Effect_Spec_Version {
            major: PF_PLUG_IN_VERSION,
            minor: PF_PLUG_IN_SUBVERS,
        },
        Property::AE_Effect_Version {
            version: std::env::var("CARGO_PKG_VERSION_MAJOR").unwrap().parse().unwrap(),
            subversion: std::env::var("CARGO_PKG_VERSION_MINOR").unwrap().parse().unwrap(),
            bugversion: std::env::var("CARGO_PKG_VERSION_PATCH").unwrap().parse().unwrap(),
            stage: Stage::Develop,
            // Distinguish ordinary/diagnostic development candidates in AE caches.
            build: if std::env::var_os("CARGO_FEATURE_PREVIEW_OVERLAY_PROBE").is_some() { 3 }
                else if std::env::var_os("CARGO_FEATURE_RENDER_DIAGNOSTICS").is_some() { 2 } else { 1 },
        },
        Property::AE_Effect_Info_Flags(0),
        Property::AE_Effect_Global_OutFlags(
            OutFlags::UseOutputExtent |
            OutFlags::NonParamVary |
            OutFlags::DeepColorAware |
            OutFlags::SendUpdateParamsUI |
            OutFlags::CustomUI
        ),
        Property::AE_Effect_Global_OutFlags_2(out_flags2),
        Property::AE_Effect_Match_Name("com.elasticgrid.fx.warp"),
        // Required by the Rust AE host entry point even for internal/test builds.
        Property::AE_Effect_Support_URL(""),
        Property::AE_Reserved_Info(0),
    ]);
}
