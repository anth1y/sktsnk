#![allow(unused_imports)]
extern crate pnet;
extern crate clap;
use clap::{App, Arg};
use tokio;
use tokio::TcpListener;
use pnet::datalink::{self, NetworkInterface};

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
// pass in addr from clap flag

fn net_addr(){

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
         .arg(
             Arg::with_name("addr")
             .help("IP Address")
             .long("addr")
             .takes_value(true),
             )
        .get_matches();
    if let Some(interface_name) = matches.value_of("iface"){
    println!("{:?}", interface_name);
    iface(interface_name.to_string());
    }
let listener = TcpListener::bind(&addr).unwap();
let port = matches.value_of("port");
let addrs = matches.value_of("addr");
}
