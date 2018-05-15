extern crate pnet;
extern crate rand;
extern crate pnet_datalink;
extern crate pustlib;

use pnet::datalink::{self};
use std::net::Ipv4Addr;
use pnet::packet::Packet;
use pnet::packet::arp::ArpPacket;
use pnet::packet::ethernet::{EtherTypes, EthernetPacket, MutableEthernetPacket};
use pnet::packet::icmpv6::Icmpv6Packet;
use pnet::packet::icmp::{echo_reply, echo_request, IcmpPacket, IcmpTypes, MutableIcmpPacket};
use pnet::packet::ip::{IpNextHeaderProtocol, IpNextHeaderProtocols};
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::ipv6::Ipv6Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::udp::UdpPacket;

use pnet::datalink::Channel::Ethernet;

use pnet_datalink::{Channel, NetworkInterface, MacAddr, ParseMacAddrErr};

use std::env;
use std::io::{self, Write};
use std::process;
use std::net::IpAddr;


//use oping::{Ping, PingResult};

fn main() {

    /* maybe use `rout -n` to figure out active interface? */
    let interface_names_match = |iface: &NetworkInterface| iface.name == "name";//iface_name;
    
    let mut command = "";

    //Never really done this before... seems bad...
    for arg in env::args() {
        match command {
            "-i" => handle_interface_request(&arg),
            "-p" => handle_ping_request(&arg),
            _ => {command = match arg.as_ref() {
                    "-i" => "-i",
                    "-p" => "-p",
                    _ => ""
                }
            }
        }
    }

    let args = env::args();

    // Find the network interface with the provided name

    let active = pustlib::interface::get_active_interface();

    let interfaces = datalink::interfaces();
    let mut active_interface : Option<datalink::NetworkInterface> = None;
    for interface in interfaces {
        if interface.ips.len() > 0 {
            active_interface = Some(interface);
        }
    }
    // Create a channel to receive on
    if let Some(active) = active {
    
        let (_, mut rx) = match datalink::channel(&active, Default::default()) {
            Ok(Ethernet(tx, rx)) => (tx, rx),
            Ok(_) => panic!("packetdump: unhandled channel type: {}"),
            Err(e) => panic!("packetdump: unable to create channel: {}", e),
        };
    }
    
}

fn handle_interface_request(arg:&String) {
    if arg.eq("list") {
        pustlib::print_interfaces();
    }
}

fn handle_ping_request(arg:&String) {
    
    

}

/*
fn main() {
    let iface_name = match env::args().nth(1) {
        Some(n) => n,
        None => {
            writeln!(io::stderr(), "USAGE: packetdump <NETWORK INTERFACE> <SOURCE IP>").unwrap();
            process::exit(1);
        },
    };

    let source_ip: Result<Ipv4Addr, AddrParseError> = match env::args().nth(2) {
        Some(n) => n.parse(),
        None => {
            writeln!(io::stderr(), "USAGE: packetdump <NETWORK INTERFACE> <SOURCE IP> <TARGET IP> <TARGET MAC>").unwrap();
            process::exit(1);
        },
    };

    let target_ip: Result<Ipv4Addr, AddrParseError> = match env::args().nth(3) {
        Some(n) => n.parse(),
        None => {
            writeln!(io::stderr(), "USAGE: packetdump <NETWORK INTERFACE> <SOURCE IP> <TARGET IP> <TARGET MAC>").unwrap();
            process::exit(1);
        },
    };

    let target_mac: Result<MacAddr, ParseMacAddrErr> = match env::args().nth(4) {
        Some(n) => n.parse(),
        None => {
            writeln!(io::stderr(), "USAGE: packetdump <NETWORK INTERFACE> <SOURCE IP> <TARGET IP> <TARGET MAC>").unwrap();
            process::exit(1);
        }
    };

    let interfaces = pnet_datalink::interfaces();
    let interfaces_name_match = |iface: &NetworkInterface| iface.name == iface_name;
    let interface = interfaces.into_iter().filter(interfaces_name_match).next().unwrap();
    let source_mac = interface.mac_address();
    let arp_operation: ArpOperation = ArpOperations::Request;

    

    send_arp_packet(interface, source_ip.unwrap(), source_mac, target_ip.unwrap(), target_mac.unwrap(), arp_operation);
    
    println!("Sent ARP packet.");
}
*/