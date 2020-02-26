extern crate pcap;

fn main() {
    let mut cap = pcap::Device::lookup().unwrap().open().unwrap();
    println!("{:?}", cap.next());
}
