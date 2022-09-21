use super::*;
use anyhow::Ok;
use howlong::Duration;
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

/// res_txs for block query transactions, and boundary check.
/// res_sigs for aggregate_sinatures of each block
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OverallResult{
    #[serde(rename = "result")]
    pub res_txs: ResultTxs,
    pub res_sigs: ResultSigs,
    pub query_param: QueryParam,
    pub query_time_ms: u64,
    pub use_inter_index: bool,
    pub use_intra_index: bool,
}

impl OverallResult {
    pub async fn verify(
        &self,
        chain: &impl LightNodeInterface
    )
    -> Result<(VerifyResult, Duration)>{
        let cpu_timer = howlong::ProcessCPUTimer::new();
        let timer = howlong::HighResolutionTimer::new();
        let res = self.inner_verify(chain).await?;
        let time = timer.elapsed();
        info!("verify used time {}",cpu_timer.elapsed());
        
        Ok((res, time))
    }

    async fn inner_verify(&self, chain: &impl LightNodeInterface) -> Result<VerifyResult>{
        let mut result = VerifyResult::default();
        let mut signature: Option<Signature>;
        let mut block_header: BlockHeader;
        let ctx = signing_context(b"");
        for (id, txs) in self.res_txs.0.iter(){
            signature = self.res_sigs.0.get(id).unwrap().to_owned();
            block_header = chain.lightnode_read_block_header(id.to_owned()).await?;
            if signature.eq(&Option::None){
                //this means no satisfying txs in block(id)
                //and the Vec stores boundary conditions 
                continue;
            }
            let mut aggre_string_txs: String = String::from("");
            let public_key = PublicKey::recover(block_header.public_key);
            for tx in txs {
                aggre_string_txs += &serde_json::to_string(&tx).unwrap();
            }
            //verify failed, malicious actions exist
            if public_key.verify(ctx.bytes(aggre_string_txs.as_bytes()), &signature.unwrap()).is_err() {
                result.add(InvalidReason::InvalidSignature);
            }
        }

        Ok(result)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResultTxs(pub HashMap<IdType, Vec<Transaction>>);

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResultSigs(pub HashMap<IdType, Option<Signature>>);

impl ResultTxs{
    pub fn new() -> Self{
        Self(HashMap::new())
    }
}

impl ResultSigs{
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
    let mut res_sigs = ResultSigs::new();

    let mut result = OverallResult {
        res_txs: res_txs.clone(),
        res_sigs: res_sigs.clone(),
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
            res_sigs.0.entry(blk_data.block_id.clone())
            .or_insert(Some(aggre_sign));
            //use intra-index or not
            if param.intra_index {
                let mut tx_id = blk_data.intra_index.get(&key).unwrap().clone();
                // In one block, the id of each tx is consecutive
                while chain.read_transaction(tx_id.clone())?.key.eq(&key) {
                    res_txs.0.entry(blk_data.block_id.clone())
                    .or_insert_with(Vec::new)
                    .push(chain.read_transaction(tx_id)?.clone());
                    tx_id += 1;
                }
            } else {
                // traverse without index
                for id in blk_data.tx_ids {
                    if chain.read_transaction(id)?.key.eq(&key) {
                        res_txs.0.entry(blk_data.block_id.clone())
                        .or_insert_with(Vec::new)
                        .push(chain.read_transaction(id)?.clone());
                    }
                }
            }
        } else {
            // intra_index is ordered & consider boundray condition
            res_sigs.0.entry(blk_data.block_id.clone())
            .or_insert(None);

            let mut txs: Vec<Transaction> = Vec::new();
            let min_id: IdType = blk_data.tx_ids[0];
            let max_id: IdType = blk_data.tx_ids.len() as IdType + min_id - 1;
            let mut tx_id: IdType = max_id.clone() + 1;
            // let keys = blk_data.intra_index.keys().;
            if param.intra_index {
                for (iter_key, id) in blk_data.intra_index.iter() {
                    if iter_key.gt(&key) {
                        tx_id = id.to_owned();
                        break;
                    }
                }
            } else {
                // traverse without index
                for id in blk_data.tx_ids.clone() {
                    if chain.read_transaction(id)?.key.gt(&key) {
                        tx_id = id.to_owned();
                        break;
                    }
                }
            }
            info!("min: {}, max: {}, ids: {}", min_id, max_id, tx_id);
            for ids in vec![tx_id - 1,tx_id].iter().map(|x| x.to_owned()) {
                info!("id:{}",ids);
                if ids >= min_id 
                && ids <= max_id {
                    txs.push(chain.read_transaction(ids)?.clone())
                }
            }

            res_txs.0.entry(blk_data.block_id.clone())
            .or_insert_with(Vec::new)
            .append(&mut txs);
        }
    }

    result.res_txs = res_txs.clone();
    result.res_sigs = res_sigs.clone();
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
    while block_index > 0 as u32 {
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