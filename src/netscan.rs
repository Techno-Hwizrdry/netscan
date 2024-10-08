use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};
use std::str::FromStr;
use std::time::Duration;
use rand::random;
use ping;

use super::hostinfo::*;

fn cidr_to_ip_addresses(cidr: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    if cidr.is_empty() {
        return Err("ERROR: Invalid CIDR format".into());
    }

    let cidr_parts: Vec<&str> = cidr.split('/').collect();
    if cidr_parts.len() != 2 {
        return Err("ERROR: Invalid CIDR format".into());
    }

    let ip_str = cidr_parts[0];
    if !ip_str.parse::<Ipv4Addr>().is_ok() {
        return Err("ERROR: Invalid IP of CIDR address".into());
    }

    let prefix_len_str = cidr_parts[1];
    if !is_valid_cidr_size(prefix_len_str) {
        return Err("ERROR: Invalid size of CIDR address".into());
    }

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

fn is_ip_reachable(ip: &str, timeout: Duration) -> bool {
    let addr = ip.parse().unwrap();
    let is_reachable = match ping::ping(
        addr,
        Some(timeout),
        Some(166),
        Some(3),
        Some(5),
        Some(&random()),
    ) {
        Ok(_) => true,
        Err(_) => false
    };

    return is_reachable;
}

fn is_valid_cidr_size(cidr_size: &str) -> bool {
    if !cidr_size.parse::<u64>().is_ok() {
        return false;
    }

    if cidr_size.is_empty() {
        return false;
    }
    
    let int = match str_to_u8(cidr_size) {
        Ok(int) => int,
        Err(_) => return false
    };
    return 1 <= int && int <= 32;
}

fn str_to_u8(s: &str) -> Result<u8, std::num::ParseIntError> {
    return s.parse::<u8>();
}

fn is_valid_ip(ip_str: &str) -> bool {
    match IpAddr::from_str(ip_str) {
        Ok(_) => true,
        Err(_) => false
    }
}

pub fn scan(ip_range: &str, ports: Vec<u16>) -> Vec<HashMap<&str, HostInfo>> {
    let mut hosts: Vec<HashMap<&str, HostInfo>> = Vec::new();

    if !ip_range.contains('/') {
        if !is_valid_ip(ip_range) {
            return vec![];
        }

        let host = scan_ports(ip_range.to_string(), ports);
        if !host.is_empty() {
            hosts.push(host);
        }
        return hosts;
    }
    
    let ips = match cidr_to_ip_addresses(ip_range){
        Ok(ips) => ips,
        Err(_) => return vec![]
    };
    for ip in ips {
        let host = scan_ports(ip, ports.clone());
        if !host.is_empty() {
            hosts.push(host);
        }
    }

    return hosts;
}

fn scan_ports(ip: String, ports: Vec<u16>) -> HashMap<&'static str, HostInfo> {
    let mut host = HashMap::new();
    let duration = Duration::from_secs(1);

    if !is_ip_reachable(&ip, duration) {
        return host;
    }

    let mut open_ports: HashMap<u16, String> = HashMap::new();
    ports.into_iter().for_each(|port| {
        let host = format!("{}:{}", ip, port);
        let socket_addr = SocketAddr::from_str(host.as_str()).unwrap();
        if let Ok(stream) = TcpStream::connect_timeout(&socket_addr, Duration::from_secs(3)) {
            open_ports.insert(port, get_banner(stream));
        }
    });

    host.insert("ip", HostInfo::Ip(ip));
    host.insert("open_parts", HostInfo::Ports(open_ports));
    return host;
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn get_banner(mut stream: TcpStream) -> String {
    let request = b"GET / HTTP/1.1\r\n\r\n";
    stream.write_all(request).unwrap();
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let thing = String::from_utf8_lossy(&buffer);
    return parse_server(&thing.to_string());
}

fn remove_blank_lines(text: &str) -> String {
    text.lines()
        .filter(|line| !line.trim().is_empty())
        .collect::<Vec<&str>>()
        .join("\n")
}

fn parse_server(banner: &str) -> String {
    let mut map = HashMap::new();
    let binding = remove_blank_lines(banner);
    for line in binding.lines() {
        if let Some(ordinal_value) = line.chars().next().map(|c| c as u32) {
            if ordinal_value == 0 {
                break;
            }
        }

        if !line.contains(':') && banner.lines().count() <= 2 {
            map.insert("Server".to_string(), line.to_string());
            continue;
        }

        if !line.to_lowercase().contains("server:") { continue }

        let mut parts = line.splitn(2, ':');
        let key = parts.next().unwrap().trim();
        let value = parts.next().unwrap().trim();

        if key == "Server" {
            map.insert(key.to_string(), value.to_string());
            break;
        }
    }

    if let Some(s) = map.get("Server") {
        return s.to_string();
    }
    return "Service info not available.".to_string();
}

#[cfg(test)]
mod tests {
    use crate::netscan::*;

    #[test]
    fn test_cidr_to_ip_addresses() -> Result<(), Box<dyn std::error::Error>> {
        let expected_ips : Vec<String> = [
            "192.168.1.2",
            "192.168.1.3"
        ].map(String::from).to_vec();
        assert_eq!(expected_ips, cidr_to_ip_addresses("192.168.1.2/31")?);
        return Ok(());
    }
    
    #[test]
    fn test_cidr_to_ip_addresses_errors() {
        // Empty CIDR
        let result = cidr_to_ip_addresses("");
        assert!(matches!(result, Err(_)));

        // Invalid CIDR format (too many parts)
        let result = cidr_to_ip_addresses("192.168.1.0/24/32");
        assert!(matches!(result, Err(_)));

        // Invalid CIDR format (too few parts)
        let result = cidr_to_ip_addresses("192.168.1.0");
        assert!(matches!(result, Err(_)));

        // Invalid IP address
        let result = cidr_to_ip_addresses("invalid/24");
        assert!(matches!(result, Err(err) if err.to_string().contains("Invalid IP")));

        // Invalid prefix length
        let result = cidr_to_ip_addresses("192.168.1.0/64");
        assert!(matches!(result, Err(_)));
    }

    #[test]
    fn test_is_ip_reachable() {
        let duration = Duration::from_secs(3);
        assert_eq!(true, is_ip_reachable("127.0.0.1", duration));
        assert_eq!(false, is_ip_reachable("10.1.1.10", duration));
    }

    #[test]
    fn test_is_valid_cidr_size() {
        assert_eq!(true, is_valid_cidr_size("1"));
        assert_eq!(true, is_valid_cidr_size("21"));
        assert_eq!(true, is_valid_cidr_size("32"));
        assert_eq!(false, is_valid_cidr_size("1k"));
        assert_eq!(false, is_valid_cidr_size(" 7"));
        assert_eq!(false, is_valid_cidr_size(""));
    }

    #[test]
    fn test_is_valid_ip() {
        assert_eq!(true, is_valid_ip("192.168.1.1"));
        assert_eq!(true, is_valid_ip("10.0.11.100"));
        assert_eq!(false, is_valid_ip("22.256.1.1"));
        assert_eq!(false, is_valid_ip("192.168,1.1"));
        assert_eq!(false, is_valid_ip("This is an invalid IP address"));
    }
}