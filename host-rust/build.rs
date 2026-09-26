use pipl::*;
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

fn main() {
    // Never let a Rust panic cross the After Effects C ABI in release builds.
    // The after-effects host macro wraps EffectMain in catch_unwind when this cfg is set.
    println!("cargo:rustc-cfg=catch_panics");
    let root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
        .parent().unwrap().to_path_buf();
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();

    let mut cpp = cc::Build::new();
    cpp.cpp(true)
        .std("c++20")
        .include(root.join("src"))
        .file(root.join("src/core/GridModel.cpp"))
        .file(root.join("src/core/GridCodec.cpp"))
        .file(root.join("src/core/WarpMath.cpp"))
        .file(root.join("src/core/CpuRenderer.cpp"))
        .file(root.join("src/bridge/elasticgrid_ffi.cpp"));

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
        generate_metal_header(&root, &out_dir);
        let mut metal = cc::Build::new();
        metal.cpp(true)
            .std("c++20")
            .include(root.join("src"))
            .include(&out_dir)
            .file(root.join("src/gpu/metal_backend.mm"))
            .flag_if_supported("-fobjc-arc")
            .flag_if_supported("-fvisibility=hidden")
            .warnings(true);
        if std::env::var("PROFILE").as_deref() == Ok("release") {
            metal.opt_level(3);
        }
        metal.compile("elasticgrid_metal");
        println!("cargo:rustc-link-lib=framework=Metal");
        println!("cargo:rustc-link-lib=framework=Foundation");
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
        "src/core/SimdPixelOps.h",
        "src/bridge/elasticgrid_ffi.cpp",
        "src/bridge/elasticgrid_ffi.h",
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

    pipl::plugin_build(vec![
        Property::Kind(PIPLType::AEEffect),
        Property::Name("ElasticGrid FX"),
        Property::Category("ElasticGrid FX"),

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
            version: 0,
            subversion: 9,
            bugversion: 0,
            stage: Stage::Develop,
            build: 1,
        },
        Property::AE_Effect_Info_Flags(0),
        Property::AE_Effect_Global_OutFlags(
            OutFlags::UseOutputExtent |
            OutFlags::NonParamVary |
            OutFlags::DeepColorAware |
            OutFlags::CustomUI
        ),
        Property::AE_Effect_Global_OutFlags_2(out_flags2),
        Property::AE_Effect_Match_Name("com.elasticgrid.fx.warp"),
        // Required by the Rust AE host entry point even for internal/test builds.
        Property::AE_Effect_Support_URL(""),
        Property::AE_Reserved_Info(0),
    ]);
}
