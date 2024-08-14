use clap::Parser;
use colored::*;
use std::collections::HashMap;
use std::num::ParseIntError;
use std::process::exit;

mod text_colors;
use text_colors::*;

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

const MAX_PORT: u16 = 65535;
const MIN_PORT: u16 = 1;
const DEFAULT_MAX: u16 = 1024;

#[derive(Parser)]
struct Cli {
    /// Either a single IP address or CIDR address.
    #[arg(short, long, required = true)]
    address: String,
    /// a single port, range of ports (ex: 21-711), or comma seperated list of ports
    #[arg(short, long)]
    ports: Option<String>
}

fn check_port_value(port: u16) {
    if port < MIN_PORT || port > MAX_PORT {
        print_and_exit("ERROR: Invalid port value.", 1);
    }
}

fn parse_comma_seperated_ports(ports: &str) -> Result<Vec<u16>, ParseIntError> {
    let mut list = vec![];
    let mut split = ports.split(",");
    loop {
        match split.next() {
            Some(p) => {
                let port: u16 = p.parse::<u16>()?;
                check_port_value(port);
                list.push(port);
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

    check_port_value(start);
    check_port_value(end);
    if start > end {
        let msg = "ERROR: starting port number must be less than ending port number.";
        print_and_exit(msg, 1);
    }

    let mut list = vec![];
    (start..=end).into_iter().for_each(|x| list.push(x));
    return Ok(list);
}

fn parse_ports_str(ports_opt: Option<String>) -> Result<Vec<u16>, String> { 
    if ports_opt.is_none() {
        return Ok((1..DEFAULT_MAX).collect::<Vec<u16>>());
    }

    let ports: &str = &ports_opt.unwrap();
    let parseint_err_msg: String = String::from("ERROR: Invalid port format");

    if ports.parse::<u16>().is_ok() {
        let single_port: u16 = ports.parse::<u16>().unwrap();
        check_port_value(single_port);
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