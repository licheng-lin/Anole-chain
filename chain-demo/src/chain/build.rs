use log::info;

use crate::Digest;
use super::*;


pub fn build_block<'a>(
    block_id: IdType,
    pre_hash: Digest,
    raw_txs: impl Iterator<Item = &'a RawTransaction>,
    chain: &mut (impl ReadInterface + WriteInterface),
) -> Result<BlockHeader> {
    info!("Build block {}", block_id);

    let param = chain.get_parameter()?;
    let txs: Vec<Transaction> = raw_txs.map(|rtx: &RawTransaction| Transaction::create(rtx)).collect();
    let mut time_stamp: TsType = Default::default();
    for tx in txs{
        time_stamp = tx.value.time_stamp; // not good enough
        chain.write_transaction(tx.clone());
    }

    let public_key: PkType = String::from("need to complete");
    
    let mut block_header = BlockHeader{
        block_id,
        pre_hash,
        time_stamp,
        public_key,
    };

    // need to complete
    let mut block_data = BlockData {
        block_id,
    };

    chain.write_block_header(block_header.clone())?;
    chain.write_block_data(block_data.clone())?;

    Ok(block_header)
}