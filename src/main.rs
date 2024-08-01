extern crate chrono;
extern crate pnet;
extern crate prettytable;
extern crate termion;

use chrono::Local;
use pnet::datalink::{self, Channel};
use pnet::packet::{ethernet::EthernetPacket, ipv4::Ipv4Packet, Packet};
use prettytable::{format, row, Cell, Row, Table};
use std::io::{self, Write};
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;
use termion::color;
use termion::input::TermRead;

struct PacketInfo {
    time: String,
    source: String,
    destination: String,
    ethertype: String,
    details: String,
    length: usize,
}

fn main() {
    loop {
        // List available network interfaces
        let interfaces = datalink::interfaces();
        println!("Available network interfaces:");
        for (index, iface) in interfaces.iter().enumerate() {
            println!("{}: {}", index, iface.name);
        }

        // Prompt the user to select an interface
        print!("Enter the number of the interface you want to use: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let index: usize = match input.trim().parse() {
            Ok(i) => i,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        let interface = match interfaces.get(index) {
            Some(i) => i,
            None => {
                println!("Invalid interface index. Please try again.");
                continue;
            }
        };

        println!("Using interface: {}", interface.name);

        match datalink::channel(interface, Default::default()) {
            Ok(Channel::Ethernet(_, mut rx)) => {
                println!("Listening on interface: {}", interface.name);

                // Channel for keyboard input
                let (tx, rx_input) = mpsc::channel();
                thread::spawn(move || {
                    let stdin = io::stdin();
                    for key in stdin.keys() {
                        let _ = tx.send(key.unwrap());
                    }
                });

                let mut paused = false;
                let mut buffer: Vec<PacketInfo> = Vec::new();

                loop {
                    // Check for keyboard input
                    match rx_input.try_recv() {
                        Ok(key) => {
                            if key == termion::event::Key::Char('\n') {
                                paused = !paused;
                                if paused {
                                    println!("Paused. Press Enter to resume.");
                                } else {
                                    println!("Resumed. Press Enter to pause.");
                                    // Flush buffer to screen
                                    for packet_info in buffer.drain(..) {
                                        print_packet(&packet_info);
                                    }
                                }
                            }
                        }
                        Err(TryRecvError::Empty) => {}
                        Err(TryRecvError::Disconnected) => break,
                    }

                    if !paused {
                        // Capture packets
                        match rx.next() {
                            Ok(packet) => {
                                let ethernet = EthernetPacket::new(packet).unwrap();
                                let source = ethernet.get_source().to_string();
                                let destination = ethernet.get_destination().to_string();
                                let ethertype = ethernet.get_ethertype().to_string();
                                let length = ethernet.packet().len();
                                let time = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();

                                let details = if ethertype
                                    == pnet::packet::ethernet::EtherTypes::Ipv4.to_string()
                                {
                                    let ipv4 = Ipv4Packet::new(ethernet.payload()).unwrap();
                                    format!(
                                        "IPv4 {} -> {}",
                                        ipv4.get_source(),
                                        ipv4.get_destination()
                                    )
                                } else {
                                    format!("{:?}", ethertype)
                                };

                                let packet_info = PacketInfo {
                                    time,
                                    source,
                                    destination,
                                    ethertype,
                                    details,
                                    length,
                                };

                                if paused {
                                    buffer.push(packet_info);
                                } else {
                                    print_packet(&packet_info);
                                }
                            }
                            Err(e) => {
                                println!("An error occurred while reading: {}", e);
                            }
                        }
                    } else {
                        thread::sleep(Duration::from_millis(100));
                    }
                }
            }
            Ok(_) => {
                println!("Unhandled channel type.");
            }
            Err(e) => {
                println!(
                    "An error occurred when creating the datalink channel: {}",
                    e
                );
            }
        }

        // Ask the user if they want to choose another interface
        print!("Do you want to choose another interface? (y/n): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if input.trim().to_lowercase() != "y" {
            break;
        }
    }

    println!("Exiting the program.");
}

fn print_packet(packet_info: &PacketInfo) {
    // Create a new table for each output to avoid accumulating rows
    let mut table = Table::new();
    table.add_row(row![
        format!("{}Time{}", color::Fg(color::White), color::Fg(color::Reset)),
        format!(
            "{}Source{}",
            color::Fg(color::Green),
            color::Fg(color::Reset)
        ),
        format!(
            "{}Destination{}",
            color::Fg(color::Blue),
            color::Fg(color::Reset)
        ),
        format!(
            "{}Type/Protocol{}",
            color::Fg(color::Yellow),
            color::Fg(color::Reset)
        ),
        format!(
            "{}Details{}",
            color::Fg(color::Magenta),
            color::Fg(color::Reset)
        ),
        format!(
            "{}Length{}",
            color::Fg(color::Cyan),
            color::Fg(color::Reset)
        )
    ]);

    // Set the table format
    table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);

    table.add_row(Row::new(vec![
        Cell::new(&format!(
            "{}{}{}",
            color::Fg(color::White),
            packet_info.time,
            color::Fg(color::Reset)
        )),
        Cell::new(&format!(
            "{}{}{}",
            color::Fg(color::Green),
            packet_info.source,
            color::Fg(color::Reset)
        )),
        Cell::new(&format!(
            "{}{}{}",
            color::Fg(color::Blue),
            packet_info.destination,
            color::Fg(color::Reset)
        )),
        Cell::new(&format!(
            "{}{:?}{}",
            color::Fg(color::Yellow),
            packet_info.ethertype,
            color::Fg(color::Reset)
        )),
        Cell::new(&format!(
            "{}{}{}",
            color::Fg(color::Magenta),
            packet_info.details,
            color::Fg(color::Reset)
        )),
        Cell::new(&format!(
            "{}{}{}",
            color::Fg(color::Cyan),
            packet_info.length,
            color::Fg(color::Reset)
        )),
    ]));

    table.printstd();
}
