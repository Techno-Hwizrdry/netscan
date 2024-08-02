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

To scan an IP given a single port:

`netscan --address 192.168.1.1 443`

To scan an IP given a range of ports:

`netscan --address 192.168.1.1 21-711`

To scan an IP given a list of ports:

`netscan --address 192.168.1.1 53,80,443,`

Omitting the -p argument will default to scanning all 65,535 ports on the target IP address (or CIDR address).

`netscan --address 192.168.1.1`

To print help:

`netscan --help`