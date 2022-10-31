pub mod grpc_health_v1 {
    include!("../target/grpc.health.v1.rs");
}

include!("../target/nvidia.riva.rs");
pub mod asr {
    include!("../target/nvidia.riva.asr.rs");
}

pub mod nlp {
    include!("../target/nvidia.riva.nlp.rs");
}

pub mod nmt {
    include!("../target/nvidia.riva.nmt.rs");
}

pub mod tts {
    include!("../target/nvidia.riva.tts.rs");
}
