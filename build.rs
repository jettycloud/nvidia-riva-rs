fn main() {
    tonic_build::configure()
        .type_attribute(
            ".",
            "#[allow(unknown_lints, clippy::derive_partial_eq_without_eq)]",
        )
        .include_file("generated.rs")
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
