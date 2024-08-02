# NetScan
A TCP port scanner written in Rust.

## Build
To build the development version:

`cargo build`

To build the release version:

`cargo build --release`

## Usage

Below are the command line arguments that can be passed to netscan.

1. **-a, --address <ADDRESS>:** Either a single IP address or CIDR address.  **This cli argument is required.**
2. **-p, --ports <PORTS>:** A single port, range of ports (ex: 21-711), or comma seperated list of ports.
3. **-h, --help:** Print help.

### Examples

Omitting the -p argument will default to scanning all 65,535 ports on the target IP address (or CIDR address).

`netscan --address 192.168.1.1`

To scan an IP given a single port:

`netscan --address 192.168.1.1 -p 443`

To scan an IP given a range of ports:

`netscan --address 192.168.1.1 -p 21-711`

To scan an IP given a list of ports:

`netscan --address 192.168.1.1 -p 53,80,443,`

Providing a CIDR address allows you to scan a range of IP addresses.  Ports can be provided too:

`netscan --address 192.168.1.1/24 -p 443`

`netscan --address 192.168.1.1/24 -p 21-711`

`netscan --address 192.168.1.1/24 -p 53,80,443`

To print help:

`netscan --help`