use clap::Parser;
use colored::*;
use std::collections::HashMap;
use std::num::ParseIntError;
use std::process::exit;

mod hostinfo;
use hostinfo::HostInfo;

mod netscan;
use netscan::scan;

const BANNER: &str = "
░▒▓███████▓▒░░▒▓████████▓▒░▒▓████████▓▒░▒▓███████▓▒░░▒▓██████▓▒░ ░▒▓██████▓▒░░▒▓███████▓▒░  
░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░         ░▒▓█▓▒░  ░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░ 
░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░         ░▒▓█▓▒░  ░▒▓█▓▒░      ░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░ 
░▒▓█▓▒░░▒▓█▓▒░▒▓██████▓▒░    ░▒▓█▓▒░   ░▒▓██████▓▒░░▒▓█▓▒░      ░▒▓████████▓▒░▒▓█▓▒░░▒▓█▓▒░ 
░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░         ░▒▓█▓▒░         ░▒▓█▓▒░▒▓█▓▒░      ░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░ 
░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░         ░▒▓█▓▒░         ░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░ 
░▒▓█▓▒░░▒▓█▓▒░▒▓████████▓▒░  ░▒▓█▓▒░  ░▒▓███████▓▒░ ░▒▓██████▓▒░░▒▓█▓▒░░▒▓█▓▒░▒▓█▓▒░░▒▓█▓▒░
";

const R: u8 = 22;
const G: u8 = 121;
const B: u8 = 226;

#[derive(Parser)]
struct Cli {
    /// Either a single IP address or CIDR address.
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

fn output_hosts(hosts: Vec<HashMap<&str, HostInfo>>) {
    if hosts.is_empty() {
        print_and_exit("\nNo hosts found.", 0);
    }
    println!(
        "{}",
        "\nIP                 Open Ports".to_string().truecolor(R, G, B)
    );
    println!(
        "{}",
        "-----------------------------".to_string().truecolor(R, G, B)
    );
    for host in hosts {
        for (_, host_info) in host {
            println!("{}", host_info);
        }
    }
}

fn main() {
    let args = Cli::parse();
    let target = args.address;
    let ports_from_user = args.ports.clone();
    let ports = parse_ports_str(args.ports);
    let mut ports_list = vec![];

    match ports {
        Ok(ref list) => ports_list=list.to_vec(),
        Err(ref error) => print_and_exit(error, 0),
    }

    println!("{}", BANNER.truecolor(R, G, B));
    println!("Target IP: {}", target.truecolor(R, G, B));
    println!(
        "Ports: {}",
        ports_from_user.unwrap_or("All ports".to_string()).truecolor(R, G, B)
    );

    let hosts = scan(&target, ports_list);
    output_hosts(hosts);
}