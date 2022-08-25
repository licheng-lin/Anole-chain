use super::{IdType, TsType, PkType, SnType};
use core::sync::atomic::{AtomicU64, Ordering};
use serde::{Deserialize, Serialize};
use crate::digest::Digest;

// static INDEX_ID_CNT: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct BlockData {
    pub block_id: IdType,
    // pub data: ,
    // pub signature: SnType,
}

//block_id == block_height, data_root = data.hash()
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct BlockHeader {
    pub block_id: IdType,
    pub pre_hash: Digest,
    // pub data_root: Digest,
    pub time_stamp: TsType,
    pub public_key: PkType,
}

impl BlockHeader {
    // signature process
    pub fn sign_transaction(&self) -> SnType {
        // self.public_key, for test purpose
        String::from("need to complete")
    }
}