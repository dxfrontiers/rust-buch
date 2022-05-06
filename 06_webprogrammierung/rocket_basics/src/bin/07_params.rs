
use structopt::StructOpt;

fn main(){
    let opt = Opt::from_args();
    dbg!(opt);
}

#[derive(StructOpt, Debug)]
#[structopt(name = "07_params")]
struct Opt {

    /// Whether output should be verbose
    #[structopt(short)]
    verbose: bool,

    /// The size of the internal queue
    #[structopt(long, short, default_value = "42")]
    queue_size: u32,

    /// Url used to connect to the database
    #[structopt(long="db", env="DB_URL")]
    db_url: String,
}