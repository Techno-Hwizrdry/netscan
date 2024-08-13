use colored::*;
use std::collections::HashMap;
use std::fmt;
use super::text_colors::*;

pub enum HostInfo {
    Ip(String),
    Ports(HashMap<u16, String>)
}

impl fmt::Display for HostInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HostInfo::Ip(ip) => write!(f, "{}", ip.truecolor(R, G, B)),
            HostInfo::Ports(ports) => {
                for (port, desc) in ports {
                    writeln!(
                        f,
                        "\t\t{} -> {}",
                        port.to_string().truecolor(_R, _G, _B),
                        desc.to_string().truecolor(_R, _G, _B)
                    )?;
                }
                return Ok(());
            }
        }
    }
}