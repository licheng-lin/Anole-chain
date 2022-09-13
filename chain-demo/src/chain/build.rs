use std::collections::{HashMap};

use log::info;

use crate::Digest;
use super::*;

///
/// 
/// For BlockData
/// --
/// intra-index is sorted & storing the first position of distinguished key.
/// 
/// aggre_signs is a series of aggregated signature based on transaction's unique key.
/// 
/// For BlockHeader
/// --
/// 
pub fn build_block<'a>(
    block_id: IdType,
    pre_hash: Digest,
    raw_txs: impl Iterator<Item = &'a RawTransaction>,
    key_pair: & Keypair,
    chain: &mut (impl ReadInterface + WriteInterface),
) -> Result<BlockHeader> {
    info!("Build block {}", block_id);
    
    // let param = chain.get_parameter()?;
    let txs: Vec<Transaction> = raw_txs.map(|rtx: &RawTransaction| Transaction::create_with_sk(rtx, &key_pair)).collect();
    let mut time_stamp: TsType = Default::default();
    let mut tx_ids: Vec<IdType> = Vec::new();
    time_stamp = txs[0].value.time_stamp;
    // intra-index & aggre_sign
    let mut intra_index: HashMap<KeyType, IdType>= HashMap::new();
    let mut aggre_signs: HashMap<KeyType, AggregateSignature> = HashMap::new(); 
    let mut signature: Vec<Signature> = Vec::new();
    let mut pre_key = txs[0].key.clone();
    let mut aggre_sign: AggregateSignature;

    for tx in txs{
        chain.write_transaction(tx.clone())?;

        if tx.key.eq(&pre_key) {
            signature.push(tx.signature);
        } else {
            aggre_sign = sign_aggregate(&signature[..]);
            aggre_signs.entry(pre_key).or_insert(aggre_sign);

            signature.clear();
            signature.push(tx.signature);
            pre_key = tx.key.clone();
        }
        
        tx_ids.push(tx.id);
        intra_index.entry(tx.key).or_insert(tx.id.clone());
        
    }
    aggre_sign = sign_aggregate(&signature[..]);
    aggre_signs.entry(pre_key).or_insert(aggre_sign);
    signature.clear();

    let public_key: PkType = key_pair.public.into_compressed();
    
    let block_header = BlockHeader{
        block_id,
        pre_hash,
        time_stamp,
        public_key,
    };

    let block_data = BlockData {
        block_id,
        tx_ids,
        intra_index,
        aggre_signs,
    };

    chain.write_block_header(block_header.clone())?;
    chain.write_block_data(block_data.clone())?;

    Ok(block_header)
}