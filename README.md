![image](https://github.com/user-attachments/assets/360690f3-0657-452e-865a-73f6caf09ca7)

# Ket_Muzzle

Ket_Muzzle is a Rust project that captures and displays network packets from a selected network interface. It uses the pnet crate to capture packets, prettytable to format the output in a table, termion for adding color to the output, and chrono to timestamp each captured packet.

## Features

- Lists all available network interfaces on the system.
- Prompts the user to select a network interface for packet capturing.
- Captures and displays Ethernet and IPv4 packets with details.
- Displays captured packets in a formatted table with color-coded columns for better readability.
- Includes packet details such as capture time, source address, destination address, type/protocol, and length.
- Allows the user to switch between network interfaces for packet capturing.

## Dependencies

The project relies on the following Rust crates:

`pnet`: For network packet capturing.
`prettytable`: For formatting output in a table.
`termion`: For adding colors to the output.
`chrono`: For adding timestamps to captured packets.
These dependencies are specified in the `Cargo.toml` file.

Clone the repository:

```sh

git clone <repository-url>
```

```sh

cd ket_muzzle
```

Build the project:

```sh

cargo build
```

Run the project in debug mode:

```sh

cargo run
```

Set capabilities for running in production:

To capture network packets, the executable needs special permissions. Set the required capabilities using setcap:

```sh

sudo setcap cap_net_raw,cap_net_admin=eip target/debug/ket_muzzle
```

Run the project in production mode:

After setting the capabilities, run the executable:

```sh

./target/debug/ket_muzzle
```

## Usage

Follow the on-screen instructions to select a network interface and start capturing packets.
The program will display captured packets in a table format with color-coded columns for better readability.
To switch to a different network interface, stop the packet capture (by pressing Ctrl+C), and the program will prompt you to choose another interface.
Example Output

```sh

Available network interfaces:
0: eth0
1: wlan0
Enter the number of the interface you want to use: 0
Using interface: eth0
Listening on interface: eth0
```

The output will display a continuously updating table with the following columns:

`Time`: The timestamp when the packet was captured.
`Source`: The source MAC address of the packet.
`Destination`: The destination MAC address of the packet.
`Type/Protocol`: The type or protocol of the packet (e.g., IPv4).
`Details`: Additional details about the packet (e.g., IPv4 source and destination addresses).
`Length`: The length of the packet.

Each column is color-coded for better readability.
