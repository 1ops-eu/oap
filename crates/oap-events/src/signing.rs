//! HMAC-SHA256 webhook payload signing and verification.

use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

/// Sign a webhook payload with HMAC-SHA256.
///
/// Returns the hex-encoded signature.
pub fn sign_payload(secret: &[u8], payload: &[u8]) -> String {
    let mut mac =
        HmacSha256::new_from_slice(secret).expect("HMAC can take key of any size");
    mac.update(payload);
    let result = mac.finalize();
    hex::encode(result.into_bytes())
}

/// Verify a webhook payload signature.
///
/// Compares the provided signature against the expected HMAC-SHA256 of the payload.
pub fn verify_signature(secret: &[u8], payload: &[u8], signature: &str) -> bool {
    let expected = sign_payload(secret, payload);
    // Constant-time comparison to prevent timing attacks
    constant_time_eq(expected.as_bytes(), signature.as_bytes())
}

/// Constant-time byte comparison.
fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}

/// Construct the webhook signature header value.
///
/// Format: `sha256=<hex-encoded-signature>`
pub fn build_signature_header(secret: &[u8], payload: &[u8]) -> String {
    let sig = sign_payload(secret, payload);
    format!("sha256={sig}")
}

/// Parse the signature from a header value.
///
/// Expects format: `sha256=<hex-encoded-signature>`
pub fn parse_signature_header(header: &str) -> Option<&str> {
    header.strip_prefix("sha256=")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign_and_verify() {
        let secret = b"test-secret";
        let payload = b"test-payload";

        let signature = sign_payload(secret, payload);
        assert!(verify_signature(secret, payload, &signature));
    }

    #[test]
    fn test_verify_wrong_secret() {
        let payload = b"test-payload";
        let signature = sign_payload(b"secret-a", payload);
        assert!(!verify_signature(b"secret-b", payload, &signature));
    }

    #[test]
    fn test_verify_wrong_payload() {
        let secret = b"test-secret";
        let signature = sign_payload(secret, b"payload-a");
        assert!(!verify_signature(secret, b"payload-b", &signature));
    }

    #[test]
    fn test_signature_header_roundtrip() {
        let secret = b"test-secret";
        let payload = b"test-payload";

        let header = build_signature_header(secret, payload);
        assert!(header.starts_with("sha256="));

        let sig = parse_signature_header(&header).unwrap();
        assert!(verify_signature(secret, payload, sig));
    }
}
