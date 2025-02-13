pub mod primitives;
pub mod types;
pub mod base64;
pub mod hex;
pub mod varint;
pub mod errors;

#[cfg(test)]
mod tests {
    mod proto;
    mod varint;
}
