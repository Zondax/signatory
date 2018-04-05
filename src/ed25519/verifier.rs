//! Trait for Ed25519 verifiers
//!
//! This is intended to be used in conjunction with the `verify` method of `PublicKey`

#[cfg(feature = "dalek-provider")]
pub use providers::dalek::DalekVerifier as DefaultVerifier;

use error::Error;
use super::{PublicKey, Signature};

/// Verifier for Ed25519 signatures
pub trait Verifier: Sync + Sized {
    /// Verify an Ed25519 signature against the given public key
    fn verify(key: &PublicKey<Self>, msg: &[u8], signature: &Signature) -> Result<(), Error>;
}

/// A panicking default verifier if no providers have been selected
#[cfg(not(feature = "dalek-provider"))]
pub struct DefaultVerifier();

#[cfg(not(feature = "dalek-provider"))]
impl Verifier for DefaultVerifier {
    fn verify(_key: &PublicKey, _msg: &[u8], _signature: &Signature) -> Result<(), Error> {
        panic!("no Ed25519 providers enabled when signatory was built");
    }
}
