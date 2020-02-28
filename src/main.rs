extern crate pnet;
extern crate clap;
use clap::App;

fn iface(i: str){
# TODO match against the string of the interface passed in via flag
}
fn main() {
    App::new("sktsnk")
        .version("0.1.0")
        .about("Does tcpdump like things")
        .author("Anthony Riley")
        .usage("sktsnk [-pidxo] <file.pnet>")
        .get_matches();
    for interface in pnet::datalink::interfaces() {
    println!("{}", interface);
    }
}
