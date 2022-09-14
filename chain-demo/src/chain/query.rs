use super::*;
use anyhow::Ok;
use log::info;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct QueryParam{

    #[serde(rename = "blk_addr")]
    pub key: KeyType,
    #[serde(rename = "time_stamp")]
    pub value: Option<[Option<TsType>; 2]>,

}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OverallResult{
    #[serde(rename = "result")]
    pub res_txs: ResultTxs,
    pub query_param: QueryParam,
    pub query_time_ms: u64,
    pub use_inter_index: bool,
    pub use_intra_index: bool,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResultTxs(pub HashMap<AggregateSignature, Vec<Transaction>>);

impl ResultTxs{
    pub fn new() -> Self{
        Self(HashMap::new())
    }
}
// #[derive(Debug, Default, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
// pub struct TimeRange([Option<TsType>; 2]);


pub fn historical_query(q_param: &QueryParam, chain: &impl ReadInterface) 
 -> Result<OverallResult>{
    info!("process query {:?}", q_param);

    let param = chain.get_parameter()?;
    let cpu_timer = howlong::ProcessCPUTimer::new();
    let timer = howlong::HighResolutionTimer::new();
    let mut res_txs = ResultTxs::new();

    let mut result = OverallResult {
        res_txs: res_txs.clone(),
        query_param: q_param.clone(),
        query_time_ms: 0,
        use_inter_index: param.inter_index,
        use_intra_index: param.intra_index,
    };
    let mut block_header: Vec<BlockHeader> = Vec::new();
    let mut block_data: Vec<BlockData> = Vec::new();

    //query block_header & block_data within the query range of timestamp
    if param.inter_index {
        
    } else {
        query_chain_no_inter_index(&q_param, &mut block_header, &mut block_data, chain)?;
    }
    //query inside block to check if consist key
    let key = q_param.key.clone();
    for blk_data in block_data {
        if blk_data.aggre_signs.contains_key(&key) {
            let aggre_sign = blk_data.aggre_signs.get(&key).unwrap().clone();
            //use intra-index or not
            if param.intra_index {
                let mut tx_id = blk_data.intra_index.get(&key).unwrap().clone();
                // In one block, the id of each tx is consecutive
                while chain.read_transaction(tx_id.clone())?.key.eq(&key) {
                    res_txs.0.entry(aggre_sign.clone())
                    .or_insert_with(Vec::new)
                    .push(chain.read_transaction(tx_id)?.clone());
                    tx_id += 1;
                } 
            } else {
                // traverse without index
                for id in blk_data.tx_ids {
                    if chain.read_transaction(id)?.key.eq(&key) {
                        res_txs.0.entry(aggre_sign.clone())
                        .or_insert_with(Vec::new)
                        .push(chain.read_transaction(id)?.clone());
                    }
                }
            }
        }
    }

    result.res_txs = res_txs.clone();
    result.query_time_ms = timer.elapsed().as_millis() as u64;
    info!("used time: {:?}", cpu_timer.elapsed());
    Ok(result)
}

// fn query_chain_inter_index()

// return BlockData & BlockHeader without checking if data consist
fn query_chain_no_inter_index(
    q_param: &QueryParam,
    block_headers: &mut Vec<BlockHeader>,
    block_datas: &mut Vec<BlockData>,
    chain: &impl ReadInterface,
) -> Result<()>{
    let mut block_index = chain.get_parameter()?.block_count.clone();
    while block_index >= 0 as u32 {
        let block_header = chain.read_block_header(block_index)?;
        let block_data = chain.read_block_data(block_index)?;
        if block_header.time_stamp >= q_param.value.unwrap()[0].unwrap()
        && block_header.time_stamp <= q_param.value.unwrap()[1].unwrap(){
            block_headers.push(block_header.to_owned());
            block_datas.push(block_data.to_owned());
        }
        
        block_index -= 1;
    }

    Ok(())
}

// fn query_block_intra_index()

// fn query_block_no_intra_index(
//     q_param: &QueryParam,
//     block_headers: & Vec<BlockHeader>,
//     block_datas: & Vec<BlockData>,
//     res_txs: &mut ResultTxs,
//     chain: &impl ReadInterface,
// ) -> Result<()>{
//     let key = q_param.key.clone();
//     for blk_data in block_datas {
//         if blk_data.aggre_signs.contains_key(&key) {

//         }
//     }

//     Ok(())
// }