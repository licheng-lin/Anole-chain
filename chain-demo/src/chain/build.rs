use std::collections::{HashMap, BTreeMap, hash_map::RandomState};

use log::info;
use rsa::pkcs8::der::Encodable;

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
    let mut intra_b_index: BTreeMap<KeyType, IdType>= BTreeMap::new();
    let intra_index: HashMap<KeyType, IdType>;
    let mut aggre_signs: HashMap<KeyType, Signature> = HashMap::new(); 
    let mut aggre_string_txs: String = String::from("");
    let mut pre_key = txs[0].key.clone();
    let mut aggre_sign: Signature;
    let ctx = signing_context(b"");

    for tx in txs{
        chain.write_transaction(tx.clone())?;

        if tx.key.eq(&pre_key) {
            aggre_string_txs += &serde_json::to_string(&tx).unwrap();
        } else {
            aggre_sign = key_pair.sign(ctx.bytes(aggre_string_txs.as_bytes()));
            aggre_signs.entry(pre_key).or_insert(aggre_sign);

            aggre_string_txs.clear();
            aggre_string_txs += &serde_json::to_string(&tx).unwrap();
            pre_key = tx.key.clone();
        }
        
        tx_ids.push(tx.id);
        intra_b_index.entry(tx.key).or_insert(tx.id.clone());
        
    }
    aggre_sign = key_pair.sign(ctx.bytes(aggre_string_txs.as_bytes()));
    aggre_signs.entry(pre_key).or_insert(aggre_sign);
    aggre_string_txs.clear();

    let public_key: PkType = key_pair.public.into_compressed();
    // convert BTreeMap to HashMap
    let keys = intra_b_index.keys();
    let values = intra_b_index.values();
    intra_index= keys.into_iter().map(|x| x.to_owned()).zip(values.into_iter().map(|y| y.to_owned())).collect::<HashMap<_,_>>();
    
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