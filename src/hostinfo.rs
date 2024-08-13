use colored::*;
use std::collections::HashMap;
use std::fmt;

const R: u8 = 22;
const G: u8 = 121;
const B: u8 = 226;
const _R: u8 = 255 - R;
const _G: u8 = 255 - G;
const _B: u8 = 255 - B;

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