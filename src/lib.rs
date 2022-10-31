/*!
GRPC Client for Nvidia Riva or Jarvis
Based on public [nvidia-riva](https://github.com/nvidia-riva/common) gRPC api
 */

/// Based on gRPC health check protocol - more details found here:
/// [health-checking](https://github.com/grpc/grpc/blob/master/doc/health-checking.md)
pub mod grpc_health_v1 {
    include!("../target/grpc.health.v1.rs");
}

include!("../target/nvidia.riva.rs");
/// Nvidia riva ASR module
pub mod asr {
    include!("../target/nvidia.riva.asr.rs");
}

/// Nvidia riva NLP module
pub mod nlp {
    include!("../target/nvidia.riva.nlp.rs");
}

/// Nvidia riva NMT module
pub mod nmt {
    include!("../target/nvidia.riva.nmt.rs");
}

/// Nvidia riva TTS module
pub mod tts {
    include!("../target/nvidia.riva.tts.rs");
}
