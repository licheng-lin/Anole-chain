use chain_demo_simchain::SimChain;
use structopt::StructOpt;
use std::path::PathBuf;
use std::fmt;
use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

static mut CHAIN: Option<SimChain> = None;

fn get_chain() -> &'static SimChain{
    unsafe{ CHAIN.as_ref().unwrap()}
}

#[derive(Debug)]
struct MyErr(anyhow::Error);

impl fmt::Display for MyErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "error: {}", self.0.to_string())
    }
}
impl actix_web::error::ResponseError for MyErr {}

fn handle_err<E: fmt::Display + fmt::Debug + Send + Sync + 'static>(e: E) -> MyErr{
    MyErr(anyhow::Error::msg(e))
}



#[derive(StructOpt, Debug)]
#[structopt(name = "simchain-server")]
struct Opts {
    /// input db path
    #[structopt(short = "-db", long, parse(from_os_str))]
    db_path: PathBuf,

    /// server binding address
    #[structopt(short, long, default_value = "127.0.0.1:8000")]
    binding: String,
}

#[actix_rt::main]
async fn main() -> actix_web::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().filter_or("RUST_LOG", "info"));
    let opts = Opts::from_args();
    let chain = SimChain::open(&opts.db_path).unwrap();
    unsafe{CHAIN = Some(chain)};
    Ok(())
}