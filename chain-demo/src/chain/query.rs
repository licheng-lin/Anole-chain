use super::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct QueryParam{

    #[serde(rename = "blk_addr")]
    pub key: KeyType,
    #[serde(rename = "time_stamp")]
    pub value: TsType, 

}