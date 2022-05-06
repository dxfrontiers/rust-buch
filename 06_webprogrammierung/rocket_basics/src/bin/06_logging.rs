#[macro_use]
extern crate log;

use env_logger::Env;

fn main (){
    //env_logger::init();
    env_logger::from_env(Env::default().default_filter_or("info")).init();
    trace!("log trace");
    debug!("log debug");
    info!("log info");
    warn!("log warn");
    error!("log error");


}