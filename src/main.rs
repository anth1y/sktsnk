extern crate pnet;
extern crate clap;
use clap::{App, Arg};
use pnet::datalink::{self, NetworkInterface};
use pnet::packet::tcp::TcpPacket;
use pnet::packet::ip::{IpNextHeaderProtocol, IpNextHeaderProtocols};
use std::net::IpAddr;

fn iface(interface_name: String ){
    let interface_names_match =
        |iface: &NetworkInterface| iface.name == interface_name;

    // Find the network interface with the provided name
    let interfaces = datalink::interfaces();
    let _interface = interfaces.into_iter()
                              .filter(interface_names_match)
                              .next()
                              .unwrap();
}

fn handle_tcp_packet(interface_name: &str, source: IpAddr, destination: IpAddr, packet: &[u8]){
    let tcp = TcpPacket::new(packet);
    if let Some(tcp) = tcp {
        println!(
            "[{}]: TCP Packet: {}:{} > {}:{}; length: {}",
            interface_name,
            source,
            tcp.get_source(),
            destination,
            tcp.get_destination(),
            packet.len()
            );
    } else {
        println!("[{}]: Malformed TCP Packet", interface_name);
    }

}
fn main() {
 let matches =   App::new("sktsnk")
        .version("0.1.0")
        .about("Does tcpdump like things")
        .author("Anthony Riley")
        .arg(
            Arg::with_name("iface")
            .help("pass network interface")
            .short("i")
            .long("iface")
            .takes_value(true),
            )
        .arg(
            Arg::with_name("port")
            .help("network port")
            .short("p")
            .long("port")
            .takes_value(true),
            )
        .get_matches();
    if let Some(interface_name) = matches.value_of("iface"){
    println!("{:?}", interface_name);
    iface(interface_name.to_string());
    }
}
