use std::io::Error;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::fs;
use std::fs::File;
use yaml_rust2::{YamlLoader};
use std::process::Command;

const DEFAULT_CONFIG: &str = 
"config:
    - address: \"127.0.0.1\"
    - port: 25565
    - script: \"./start.sh\"";

fn assure_config() -> Result<(), Error> {
    if fs::exists("config.yaml")? {
        return Ok(())
    }

    let mut config_file = File::create("config.yaml")?;
    config_file.write_all(DEFAULT_CONFIG.as_bytes())?;

    Ok(())
}

fn fake_response(stream: &mut TcpStream, script_path_str: &str) -> Result<(), Error> {

    let mut buf: [u8; 128] = [0; 128];
    stream.read(&mut buf)?;

    //println!("{:?}", buf);

    if buf[0] == 16 {
        print!("Request received: ");

        if buf[16] == 2 {
            print!(" Login attempt detected!\n");
            start_script(script_path_str)?;
            return Ok(());
        }

        print!(" Something else!\n");
    }

    

    Ok(())
}

fn start_script(script_path_str: &str) -> Result<(), Error> {
    if !fs::exists(script_path_str)? {
        let file_content = "echo Your script didn't exist so I created it :3".as_bytes();
        let mut file = File::create(script_path_str)?;
        file.write_all(file_content)?;
    }

    Command::new("chmod").arg("+x").arg(format!("{script_path_str}")).output()?;
    Command::new(format!("{script_path_str}")).output()?;
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
        fake_response(&mut stream?, script)?;
    }

    Ok(())
}