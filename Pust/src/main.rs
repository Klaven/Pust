extern crate pnet;
extern crate rand;
extern crate pnet_datalink;

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

    /*let iface_name = match env::args().nth(1) {
        Some(n) => n,
        None => {
            writeln!(io::stderr(), "USAGE: packetdump <NETWORK INTERFACE>").unwrap();
            process::exit(1);
        },
    };*/
    let interface_names_match = |iface: &NetworkInterface| iface.name == "Marek";//iface_name;
    
    let mut command = "";

    for arg in env::args() {
        match command {
            "-i" => handle_interface_request(&arg),
            _ => {command = match arg.as_ref() {
                    "-i" => "-i",
                    _ => ""
                }
            }
        }
    }

    let args = env::args();

    // Find the network interface with the provided name



    let interfaces = datalink::interfaces();
    let interface = interfaces.into_iter().filter(interface_names_match).next().unwrap();

    // Create a channel to receive on
    let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("packetdump: unhandled channel type: {}"),
        Err(e) => panic!("packetdump: unable to create channel: {}", e),
    };
    
}

fn handle_interface_request(arg:&String) {
    if arg.eq("list") {

        for interfaces in datalink::interfaces() {
            println!("{}", interfaces);
        }
    }
}

fn send_icmp_packet(interface: NetworkInterface, source_ip: Ipv4Addr, source_mac: MacAddr, target_ip: Ipv4Addr, target_mac: MacAddr) {
       let(mut tx, _) = match pnet_datalink::channel(&interface, Default::default()) {
        Ok(Channel::Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unknown channel type"),
        Err(e) => panic!("Error happened {}", e),
    };

    let mut buff = [0,0,0,0,0,0,0,0];
    let mut packet = MutableIcmpPacket::new( &mut buff);

    
    
    //let(mut tx, rx) = match pnet_datalink::channel(network_interface, configuration);

    /*
    let(mut tx, _) = match pnet_datalink::channel(&interface, Default::default()) {
        Ok(Channel::Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unknown channel type"),
        Err(e) => panic!("Error happened {}", e),
    };

    let mut ethernet_buffer = [0u8; 42];
    let mut ethernet_packet = MutableEthernetPacket::new(&mut ethernet_buffer).unwrap();

    ethernet_packet.set_destination(target_mac);
    ethernet_packet.set_source(source_mac);
    ethernet_packet.set_ethertype(EtherTypes::Arp);

    let mut arp_buffer = [0u8; 28];
    let mut arp_packet = MutableArpPacket::new(&mut arp_buffer).unwrap();

    arp_packet.set_hardware_type(ArpHardwareTypes::Ethernet);
    arp_packet.set_protocol_type(EtherTypes::Ipv4);
    arp_packet.set_hw_addr_len(6);
    arp_packet.set_proto_addr_len(4);
    arp_packet.set_operation(arp_operation);
    arp_packet.set_sender_hw_addr(source_mac);
    arp_packet.set_sender_proto_addr(source_ip);
    arp_packet.set_target_hw_addr(target_mac);
    arp_packet.set_target_proto_addr(target_ip);

    ethernet_packet.set_payload(arp_packet.packet_mut());

    tx.send_to(ethernet_packet.packet(), Some(interface));
    */
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