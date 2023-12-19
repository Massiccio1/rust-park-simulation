use std::{io, str::FromStr};
use config::Config;
use std::collections::HashMap;

fn main() -> Result<(), std::io::Error>{
    let settings = Config::builder()
        .add_source(config::File::with_name("../config/config.toml"))
        .build()
        .unwrap();

    println!("{}", settings.get_string("key").unwrap());
    println!("{}", settings.get_int("priority").unwrap());

    return Ok(());
}
