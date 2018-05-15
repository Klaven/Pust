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

use pnet::datalink::{self};

pub fn handle_interface_request(arg:&String) {
    if arg.eq("list") {

        for interfaces in datalink::interfaces() {
            println!("{}", interfaces);
        }
    }
}

pub fn get_active_interface() -> Option<pnet_datalink::NetworkInterface> {

    let interfaces = datalink::interfaces();

    let mut active_interface : Option<datalink::NetworkInterface> = None;

    //currently just selects the last one in the list.
    for inter in interfaces {
        let thing = inter.mac_address();
        let thing2 = inter.is_up();
        let thing3 = inter.ips.first();
        
        if let Some(thing3) = thing3 {
            println!("{}",thing3);
        }
        if (!inter.is_loopback()) {
            active_interface = Some(inter.clone());
        }
        
    }

    return active_interface; 
}



fn send_icmp_packet(interface: NetworkInterface, source_ip: Ipv4Addr, source_mac: MacAddr, target_ip: Ipv4Addr, target_mac: MacAddr) {
       
    let(mut tx, rx) = match pnet_datalink::channel(&interface, Default::default()) {
        Ok(Channel::Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unknown channel type"),
        Err(e) => panic!("Error happened {}", e),
    };

    let mut buff = [0,0,0,0,0,0,0,0];
    let mut packet = MutableIcmpPacket::new( &mut buff);

    
    
    let(mut tx, rx) = match pnet_datalink::channel(network_interface, configuration);

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
}