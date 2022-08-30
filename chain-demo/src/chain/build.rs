use super::*;


pub fn build_block<'a>(
    block_id: IdType,
    pre_hash: Digest,
    raw_txs: impl Iterator<Item = &'a RawObject>,
    chain: &mut (impl ReadInterface + WriteInterface),
) -> Result<BlockHeader> {
    info!("Build block {}", block_id);

    let param = chain.get_parameter()?;
    let txs: Vec<Transaction> = raw_txs.map(|rtx: &RawTransaction| Transaction::create(rtx)).collect();
    let timestamp: TsType = Default::default();
    for tx in txs{
        timestamp = tx.value.time_stamp; // not good enough
        chain.write_transaction(tx.clone());
    }

    let pub_key: PkType = String::from("need to complete");
    
    let mut block_header = BlockHeader{
        block_id,
        pre_hash,
        timestamp,
        pub_key,
    };

    // need to complete
    let mut block_data = BlockData {
        block_id,
    };

    chain.write_block_header(block_header)?;
    chain.write_block_data(block_data)?;

    Ok(block_header)
}