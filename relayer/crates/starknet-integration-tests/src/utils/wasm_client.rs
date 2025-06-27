use std::io::Write;

use flate2::write::GzEncoder;
use flate2::Compression;
use hermes_cosmos::error::Error;
use sha2::{Digest, Sha256};

pub async fn load_wasm_client(wasm_client_code_path: &str) -> Result<([u8; 32], Vec<u8>), Error> {
    let wasm_client_byte_code = tokio::fs::read(&wasm_client_code_path).await?;

    let wasm_code_hash: [u8; 32] = {
        let mut hasher = Sha256::new();
        hasher.update(&wasm_client_byte_code);
        hasher.finalize().into()
    };

    let wasm_client_byte_code_gzip = {
        let mut encoder = GzEncoder::new(Vec::new(), Compression::best());
        encoder.write_all(&wasm_client_byte_code)?;
        encoder.finish()?
    };

    Ok((wasm_code_hash, wasm_client_byte_code_gzip))
}
