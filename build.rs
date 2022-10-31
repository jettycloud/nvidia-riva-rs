use std::path::PathBuf;

fn main() {
    tonic_build::configure()
        .out_dir(default_proto_dir().unwrap())
        .type_attribute(
            ".",
            "#[allow(unknown_lints, clippy::derive_partial_eq_without_eq)]",
        )
        .compile(
            &[
                "api/common/riva/proto/health.proto",
                "api/common/riva/proto/riva_audio.proto",
                "api/common/riva/proto/riva_asr.proto",
                "api/common/riva/proto/riva_nlp.proto",
                "api/common/riva/proto/riva_nmt.proto",
                "api/common/riva/proto/riva_tts.proto",
            ],
            &["api/common/"],
        )
        .unwrap();
}

pub fn default_proto_dir() -> anyhow::Result<PathBuf> {
    let dir = format!("{}/target", std::env::var("CARGO_MANIFEST_DIR")?).into();
    std::fs::create_dir_all(&dir)?;
    Ok(dir)
}