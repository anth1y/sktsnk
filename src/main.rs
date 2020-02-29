extern crate pnet;
extern crate clap;
use clap::{App, Arg};
use pnet::datalink::{self, NetworkInterface};


fn iface(){
    let interface_name = "foo"; // pass in interface from clap
    let interface_names_match =
        |iface: &NetworkInterface| iface.name == interface_name;

    // Find the network interface with the provided name
    let interfaces = datalink::interfaces();
    let interface = interfaces.into_iter()
                              .filter(interface_names_match)
                              .next()
                              .unwrap();
}
fn main() {
    App::new("sktsnk")
        .version("0.1.0")
        .about("Does tcpdump like things")
        .author("Anthony Riley")
        .arg(
            Arg::with_name("iface")
            .help("pass network interface")
            .short('i')
            .long("iface")
            .takes_value(true),
            )
        .arg(
            Arg::with_name("port")
            .help("network port")
            .short('p')
            .long("port")
            .takes_value(true),
            )
        .get_matches();
    for interface in pnet::datalink::interfaces() {
    println!("{}", interface);
    }
}
