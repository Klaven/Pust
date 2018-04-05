extern crate pnet;
extern crate rand;

use pnet::datalink::{self, NetworkInterface};
use std::net::Ipv4Addr;
use pnet::packet::Packet;
use pnet::packet::arp::ArpPacket;
use pnet::packet::ethernet::{EtherTypes, EthernetPacket, MutableEthernetPacket};
use pnet::packet::icmpv6::Icmpv6Packet;
use pnet::packet::icmp::{echo_reply, echo_request, IcmpPacket, IcmpTypes};
use pnet::packet::ip::{IpNextHeaderProtocol, IpNextHeaderProtocols};
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::ipv6::Ipv6Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::udp::UdpPacket;
use pnet::util::MacAddr;

use std::env;
use std::io::{self, Write};
use std::process;
use std::net::IpAddr;

#[derive(Debug)]
struct Ping {
    id: u32,
    sequence: u32,
    addresses: Vec,
    address: Ipv4Addr,
    network: &str,
    source: &str,
    source6: &str,
    hasIPv4: bool,
    timeout: u32,

    onReceived: fn(&Ipv4Addr, i64)
}

impl Ping {
    // add code here
}

fn new_ping(source_ip:&Ipv4Addr) -> &Ping {
    Ping {
        id: rng.gen::<u32>(),
        sequence: rng.gen::<u32>(),
        addresses : Vec::new(),
        network: "ip",
        address: source_ip,
        source: "",
        source6: "",
        has_ip_v4: true,
        timeout: 5
        }
}
