
#[macro_use]
extern crate std;
 
fn rand_hack() -> impl RngCore+CryptoRng {
    rand_core::OsRng
}

#[macro_use]
mod serdey;
mod scalars;
use rand_core::{RngCore,CryptoRng};
pub mod keys;
pub mod errors;
pub mod points;
mod batch;
pub mod context;
mod boscoster;
pub mod sign;

pub use crate::errors::{SignatureError,SignatureResult};
pub use crate::context::{signing_context}; // SigningContext,SigningTranscript
pub use crate::sign::{Signature,SIGNATURE_LENGTH,sign_aggregate};
pub use crate::keys::*;
pub use crate::batch::{verify_batch,verify_batch_rng,verify_batch_deterministic,PreparedBatch,verify_batch_direct,verify_batch_bos};
