use structopt::StructOpt;
use std::path::PathBuf;


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

    Ok(())
}