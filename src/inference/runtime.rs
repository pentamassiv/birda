//! ONNX Runtime startup checks.

use crate::error::{Error, Result};

/// Ensure ONNX Runtime is initialized before inference-related code touches `ort`.
///
/// When `ort` is built with its `download-binaries` feature, the ONNX Runtime
/// library is provided by the `ort` crate itself. No external
/// `libonnxruntime.so` lookup is necessary.
pub fn ensure_runtime_available() -> Result<()> {
    ort::init()
        .commit()
       ;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initializes_runtime() {
        ensure_runtime_available().expect("ONNX Runtime should initialize");
    }
}