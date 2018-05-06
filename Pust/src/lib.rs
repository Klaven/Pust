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


pub mod ping;

pub mod interface;

pub fn print_interfaces() {
    for interfaces in datalink::interfaces() {
        println!("{}", interfaces);
    }
}
