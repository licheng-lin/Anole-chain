
// #[macro_use]
// extern crate std;
 
fn rand_hack() -> impl RngCore+CryptoRng {
    rand_core::OsRng
}

#[macro_use]
pub mod serdey;
use curve25519_dalek::{scalar::Scalar, ristretto::CompressedRistretto};
use serde::{Serialize, Deserialize};
pub use serdey::*;

pub mod scalars;

pub use rand_core::{RngCore,CryptoRng};
pub mod keys;
pub use keys::*;

pub mod errors;
pub use errors::*;

pub mod points;
pub use points::*;

pub mod batch;
pub use batch::*;

pub mod context;
pub use context::*;

pub mod boscoster;
pub use boscoster::*;

pub mod sign;
pub use sign::*;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct AggregateSignature(Scalar, Vec<CompressedRistretto>, CompressedRistretto);
