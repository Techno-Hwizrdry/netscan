use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn cidr_to_ip_addresses(cidr: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let cidr_parts: Vec<&str> = cidr.split('/').collect();
    if cidr_parts.len() != 2 {
        return Err("ERROR: Invalid CIDR format".into());
    }

    let ip_str = cidr_parts[0];
    let prefix_len_str = cidr_parts[1];
    let ip: Ipv4Addr = ip_str.parse().expect("parse failed");
    let prefix_len = prefix_len_str.parse::<u8>()?;

    let ip_num = u32::from_be_bytes(ip.octets());

    let mask = !((1 << (32 - prefix_len)) -1);
    let network_addr = ip_num & mask;
    let broadcast_addr = network_addr | !mask;

    let mut ip_addresses = Vec::new();
    for ip_num in network_addr..=broadcast_addr {
        let ip = Ipv4Addr::from(ip_num.to_be_bytes());
        ip_addresses.push(ip.to_string());
    }

    return Ok(ip_addresses);
}

fn is_valid_ip(ip_str: &str) -> bool {
    match IpAddr::from_str(ip_str) {
        Ok(_) => true,
        Err(_) => false
    }
}

pub fn scan(ip_range: &str, ports: Vec<u16>) -> Vec<HashMap<&str, String>> {
    let mut hosts: Vec<HashMap<&str, String>> = Vec::new();

    if !ip_range.contains('/') {
        if !is_valid_ip(ip_range) {
            return vec![];
        }

        hosts.push(scan_ports(ip_range.to_string(), ports));
        return hosts;
    }
    
    let ips = match cidr_to_ip_addresses(ip_range){
        Ok(ips) => ips,
        Err(_) => return vec![]
    };
    for ip in ips {
        let host = scan_ports(ip, ports.clone());
        hosts.push(host)
    }

    return hosts;
}

fn scan_ports(ip: String, ports: Vec<u16>) -> HashMap<&'static str, String> {
    let open_ports: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));

    ports.into_iter().for_each(|port| {
        let host = format!("{}:{}", ip, port);
        let open_ports = Arc::clone(&open_ports);
        let socket_addr = SocketAddr::from_str(host.as_str()).unwrap();
        if let Ok(_) = TcpStream::connect_timeout(&socket_addr, Duration::from_secs(3)) {
            open_ports.lock().unwrap().push(port.to_string());
        }
    });

    let mut host = HashMap::new();
    host.insert("ip", ip);
    host.insert("open_ports", open_ports.lock().unwrap().join(", "));
    return host;
}