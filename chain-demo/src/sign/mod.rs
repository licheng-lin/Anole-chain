
// #[macro_use]
// extern crate std;
 
fn rand_hack() -> impl RngCore+CryptoRng {
    rand_core::OsRng
}

#[macro_use]
pub mod serdey;
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


// pub use errors::{SignatureError,SignatureResult};
// pub use context::{signing_context}; // SigningContext,SigningTranscript
// pub use sign::{Signature,SIGNATURE_LENGTH,sign_aggregate};
// pub use keys::*;
// pub use batch::{verify_batch,verify_batch_rng,verify_batch_deterministic,PreparedBatch,verify_batch_direct,verify_batch_bos};
// pub use sign::{PublicKey,Keypair,Signature, signing_context,verify_batch_bos,sign_aggregate,verify_batch};
