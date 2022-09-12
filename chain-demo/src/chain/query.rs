use super::*;
use curve25519_dalek::scalar::Scalar;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct QueryParam{

    #[serde(rename = "blk_addr")]
    pub key: KeyType,
    #[serde(rename = "time_stamp")]
    pub value: TsType, 

}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OverallResult{
    #[serde(rename = "result")]
    pub res_txs: ResultTxs,
    pub query_param: QueryParam,
    pub query_time_ms: u64,
    pub use_index: bool,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResultTxs(pub HashMap<AggregateSign, Vec<Transaction>>);

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct AggregateSign(pub Scalar, pub Vec<CompressedRistretto>, pub CompressedRistretto);