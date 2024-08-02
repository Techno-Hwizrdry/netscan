use clap::Parser;
use std::num::ParseIntError;
use std::process::exit;

const TITLE: &str = "
░▒▓███████▓▒░░▒▓████████▓▒░▒▓████████▓▒░▒▓███████▓▒░░▒▓██████▓▒░ ░▒▓██████▓▒░░▒▓███████▓▒░  
░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░         ░▒▓█▓▒░  ░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░ 
░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░         ░▒▓█▓▒░  ░▒▓█▓▒░      ░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░ 
░▒▓█▓▒░░▒▓█▓▒░▒▓██████▓▒░    ░▒▓█▓▒░   ░▒▓██████▓▒░░▒▓█▓▒░      ░▒▓████████▓▒░▒▓█▓▒░░▒▓█▓▒░ 
░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░         ░▒▓█▓▒░         ░▒▓█▓▒░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░ 
░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░         ░▒▓█▓▒░         ░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░ 
░▒▓█▓▒░░▒▓█▓▒░▒▓████████▓▒░  ░▒▓█▓▒░  ░▒▓███████▓▒░ ░▒▓██████▓▒░░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░
";

#[derive(Parser)]
struct Cli {
    /// address
    #[arg(short, long, required = true)]
    address: String,
    /// a single port, range of ports (ex: 21-711), or comma seperated list of ports
    #[arg(short, long)]
    ports: Option<String>
}

fn parse_comma_seperated_ports(ports: &str) -> Result<Vec<u16>, ParseIntError> {
    let mut list = vec![];
    let mut split = ports.split(",");
    loop {
        match split.next() {
            Some(p) => {
                list.push(p.parse::<u16>()?);
            }
            None => break
        }
    }

    return Ok(list);
}

fn parse_port_range(ports: &str) -> Result<Vec<u16>, ParseIntError> {
    let mut split = ports.split("-");
    let start: u16 = match split.next() {
        Some(v) => v.parse::<u16>()?,
        None => panic!("range value is not valid, see netscan --help for more info")
    };
    let end: u16 = match split.next() {
        Some(v) => v.parse::<u16>()?,
        None => panic!("range value is not valid, see netscan --help for more info")
    };
    let mut list = vec![];
    (start..=end).into_iter().for_each(|x| list.push(x));
    return Ok(list);
}

fn parse_ports_str(ports_opt: Option<String>) -> Result<Vec<u16>, String> { 
    if ports_opt.is_none() {
        return Ok((1..65535).collect::<Vec<u16>>());
    }

    let ports: &str = &ports_opt.unwrap();
    let parseint_err_msg: String = String::from("ERROR: Invalid port format");

    if ports.parse::<u16>().is_ok() {
        let single_port: u16 = ports.parse::<u16>().unwrap();
        return Ok(vec![single_port]);
    }

    if ports.contains(",") {
        let result = parse_comma_seperated_ports(ports);
        
        match result {
            Ok(list) => return Ok(list),
            Err(_) => return Err(parseint_err_msg)
        };
    }

    if ports.contains("-") {
        let result = parse_port_range(ports);

        match result {
            Ok(list) => return Ok(list),
            Err(_) => return Err(parseint_err_msg)
        }
    }

    Err(parseint_err_msg)
}

fn print_and_exit(msg: &str, code: i32) {
    println!("{}", msg);
    exit(code);
}

fn main() {
    let args = Cli::parse();
    let target = args.address;
    let ports = parse_ports_str(args.ports);
    let mut ports_list = vec![];

    match ports {
        Ok(ref list) => ports_list=list.to_vec(),
        Err(ref error) => print_and_exit(error, 0),
    }

    println!("{}", TITLE);
    println!("Target IP: {}", target);
    println!("Ports: {:?}", ports_list)
}