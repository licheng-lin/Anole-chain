
use anyhow::Result;
use curve25519_dalek::ristretto::CompressedRistretto;
use serde::{Serialize, Deserialize};
use super::*;

pub mod transaction;
pub use transaction::*;

pub mod index;
pub use index::*;

pub mod utils;
pub use utils::*;

pub mod build;
pub use build::*;

pub mod query;
pub use query::*;

pub mod verify;
pub use verify::*;

pub type IdType = u64;
// Timestamp size 4 bytes
pub type TsType = u64; 
// public key size 4 bytes
pub type PkType = CompressedRistretto;
// private key 
// pub type SkType = RsaPrivateKey;
// signature
pub type SnType = Signature;
//key
pub type KeyType = String;
//transaction valßßue
pub type Txtype = u64;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Parameter {
    pub error_bounds: u8,
    pub inter_index: bool,
    pub intra_index: bool,
    pub start_block_index: u64,
    pub block_count: u64,
}

#[async_trait::async_trait]
pub trait LightNodeInterface {
    async fn lightnode_get_parameter(&self) -> Result<Parameter>;
    async fn lightnode_read_block_header(&self, id: IdType) -> Result<BlockHeader>;
}

pub trait ReadInterface {
    fn get_parameter(&self) -> Result<Parameter>;
    fn read_block_header(&self, id: IdType) -> Result<BlockHeader>;
    fn read_block_data(&self, id: IdType) -> Result<BlockData>;
    // fn read_intra_index_node(&self, id: IdType) -> Result<IntraIndexNode>;
    // fn read_skip_list_node(&self, id: IdType) -> Result<SkipListNode>;
    fn read_transaction(&self, id: IdType) -> Result<Transaction>;
}

pub trait WriteInterface {
    fn set_parameter(&mut self, param: Parameter) -> Result<()>;
    fn write_block_header(&mut self, header: BlockHeader) -> Result<()>;
    fn write_block_data(&mut self, data: BlockData) -> Result<()>;
    // fn write_intra_index_node(&mut self, node: IntraIndexNode) -> Result<()>;
    // fn write_skip_list_node(&mut self, node: SkipListNode) -> Result<()>;
    fn write_transaction(&mut self, tx: Transaction) -> Result<()>;
}

#[cfg(test)]
mod tests;
