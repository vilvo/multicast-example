// adapted from https://github.com/libpnet/libpnet/blob/master/examples/packetdump.rs
use pnet::datalink::{self, NetworkInterface};
use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
use pnet::packet::Packet;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::ip::{IpNextHeaderProtocol, IpNextHeaderProtocols};
use pnet::packet::udp::UdpPacket;

use std::net::{IpAddr,Ipv4Addr};

const INTERFACE: &str = "eth0";
const SMA_HM2_MULTICAST_IP: Ipv4Addr = Ipv4Addr::new(239,12,255,254);

fn handle_udp_packet(interface_name: &str, source: IpAddr, destination: IpAddr, packet: &[u8]) {
    let udp = UdpPacket::new(packet);

    if let Some(udp) = udp {
		if destination == SMA_HM2_MULTICAST_IP {
	        println!(
	            "[{}]: UDP Packet: {}:{} > {}:{}; length: {}:",
	            interface_name,
	            source,
	            udp.get_source(),
	            destination,
	            udp.get_destination(),
	            udp.get_length()
	        );
	    }
    } else {
        println!("[{}]: Malformed UDP Packet", interface_name);
    }
}

fn handle_transport_protocol(
    interface_name: &str,
    source: IpAddr,
    destination: IpAddr,
    protocol: IpNextHeaderProtocol,
    packet: &[u8],
) {
	if protocol == IpNextHeaderProtocols::Udp {
		handle_udp_packet(interface_name, source, destination, packet)
	}
}

fn handle_ipv4_packet(interface_name: &str, ethernet: &EthernetPacket) {
    let header = Ipv4Packet::new(ethernet.payload());
    if let Some(header) = header {
        handle_transport_protocol(
            interface_name,
            IpAddr::V4(header.get_source()),
            IpAddr::V4(header.get_destination()),
            header.get_next_level_protocol(),
            header.payload(),
        );
    } else {
        println!("[{}]: Malformed IPv4 Packet", interface_name);
    }
}

fn handle_ethernet_frame(interface: &NetworkInterface, ethernet: &EthernetPacket) {
    let interface_name = &interface.name[..];
    if ethernet.get_ethertype() == EtherTypes::Ipv4 {
		handle_ipv4_packet(interface_name, ethernet);
    }
}

fn main() {
	use pnet::datalink::Channel::Ethernet;

	let interface_match = |iface: &NetworkInterface| iface.name == INTERFACE;
	let interfaces = datalink::interfaces();
	let interface = interfaces.into_iter().find(interface_match)
		                .unwrap_or_else(||panic!("No matching interface"));

	let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
		Ok(Ethernet(tx,rx)) => (tx, rx),
		Ok(_) => panic!("unhandled channel"),
		Err(e) => panic!("error creating channel: {}", e),
	};

	loop {
		match rx.next() {
			Ok(packet) => {
				handle_ethernet_frame(&interface, &EthernetPacket::new(packet).unwrap());
			}
			Err(e) => panic!("error receiving packet: {}", e),
		}
	}
}
