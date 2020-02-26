extern crate pcap;
extern crate clap;
use clap::App;

fn main() {
    App::new("sktsnk")
        .version("0.1.0")
        .about("Does tcpdump like things")
        .author("Anthony Riley")
        .usage("sktsnk [port src dest dev out] <file.pcap>")
        .get_matches();
    let mut cap = pcap::Device::lookup().unwrap().open().unwrap();
    println!("{:?}", cap.next());
}
