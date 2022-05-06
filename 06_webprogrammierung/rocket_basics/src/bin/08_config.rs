use serde::Deserialize;
use std::{io, fs};

#[derive(Deserialize, Debug)]
struct Config{
    api_key: String,
    some_params: Option<String>,
    elements: Option<Vec<ConfigElement>>
}
#[derive(Deserialize, Debug)]
struct ConfigElement{
    foo: String,
}


fn main() -> io::Result<()>{
    let config_string = fs::read_to_string("Config.toml")?;
    let config: Config = toml::from_str(&config_string)?;
    dbg!(config);
    Ok(())
}
