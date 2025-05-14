use std::io::Error;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::fs;
use std::fs::File;
use yaml_rust2::{YamlLoader, YamlEmitter};

const default_config: &str = 
"config:
    - address: \"127.0.0.1\"
    - port: 25565
    - script: \"start.sh\"";

fn assure_config() -> Result<(), Error> {
    if fs::exists("config.yaml")? {
        return Ok(())
    }

    let mut config_file = File::create("config.yaml")?;
    config_file.write_all(default_config.as_bytes())?;

    Ok(())
}

fn fake_response() -> Result<(), Error> {
    
}

fn start_script() -> Result<(), Error> {

    let mut buf: [u8; 128] = [0; 128];
    stream.read(&mut buf)?;

    Ok(())
}

fn temp_test(mut stream: TcpStream) -> Result<(), Error> {

    let mut buf: [u8; 128] = [0; 128];
    stream.read(&mut buf)?;

    println!("{:?}", buf);

    Ok(())
}

fn main() -> Result<(), Error> {
    let address: &str;
    let port: u32;
    let script: &str;

    assure_config()?;

    let config_file = fs::read_to_string("config.yaml")?;
    let config_yaml = YamlLoader::load_from_str(config_file.as_str()).unwrap();

    address = config_yaml[0]["config"][0]["address"].as_str().unwrap();
    port = config_yaml[0]["config"][1]["port"].as_i64().unwrap() as u32;
    script = config_yaml[0]["config"][2]["script"].as_str().unwrap();

    let listener = TcpListener::bind(format!("{address}:{port}"))?;
    println!("Listening on {address}:{port} . . .");

    for stream in listener.incoming() {
        println!("Request received! {counter}");
        fake_response(stream?)?;
        start_script()?;
    }

    Ok(())
}