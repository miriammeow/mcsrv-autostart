use std::io::Error;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::fs;
use std::fs::File;
use yaml_rust2::{YamlLoader, YamlEmitter};

const default_config: &str = 
"
config:
    - address: \"0.0.0.0\"
    - port: 25565
    - script: \"start.sh\"
";

fn assure_config() -> Result<(), Error> {
    if fs::exists("config.yaml")? {
        return Ok(())
    }

    let mut config_file = File::create("config.yaml")?;
    config_file.write_all(default_config.as_bytes())?;

    Ok(())
}

fn main() -> Result<(), Error> {
    let port: u32;
    let address: &str;
    let script: &str;

    assure_config()?;

    let config_file = fs::read_to_string("config.yaml")?;
    let config_yaml = YamlLoader::load_from_str(config_file.as_str()).unwrap();
    println!();

    Ok(())
}