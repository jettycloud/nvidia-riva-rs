//! GRPC Client for Nvidia Riva or Jarvis
//! 
//! Based on public [nvidia-riva](https://github.com/nvidia-riva/common) gRPC api

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

/// Re-exporting the main module for a better developer experience.
pub use crate::nvidia::riva::*;
/// Based on gRPC health check protocol - more details found here:
/// [health-checking](https://github.com/grpc/grpc/blob/master/doc/health-checking.md)
pub use crate::grpc::health::v1 as grpc_health_v1;
