#[macro_use]
extern crate log;

use anyhow::{Result};
use std::path::{Path, PathBuf};
use chain_demo::*;
use chain_demo_simchain::SimChain;



fn build_chian(data_path: &Path, out_db_path: &Path, param: &Parameter) -> Result<()> {
    info!("build chain using data from {:?}", data_path);
    info!("output db path: {:?}",out_db_path);
    info!("param: {:?}",param);

    let raw_txs = load_raw_tx_from_file(data_path)?;
    let mut chain = SimChain::create(out_db_path, param.clone())?;
    chain.set_parameter(param.clone())?;

    let mut pre_hash = Digest::default();
    for (id,tx) in raw_txs.iter(){
        info!("build block {}", id);
        
    }
    Ok(())
}



fn main() -> Result<()> {
    env_logger::init_from_env(env_logger::Env::default().filter_or("RUST_LOG", "info"));


    Ok(())
}