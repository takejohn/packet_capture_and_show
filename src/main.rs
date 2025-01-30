use pnet::datalink::{self, Channel::Ethernet};
use pnet::packet::ethernet::EthernetPacket;
use std::{env, process};

fn main() {
    // 使用するネットワークインターフェースを選択
    let interfaces = datalink::interfaces();
    let Some(interface_name) = env::args().nth(1) else {
        eprintln!("Usage: cargo run -- <interface_name>");
        eprintln!("Available network interfaces:");
        for interface in interfaces {
            eprintln!("- {}", interface.name);
        }
        process::exit(1);
    };

    let interface = interfaces
        .into_iter()
        .find(|iface| iface.name == interface_name)
        .expect("Interface not found");

    // パケットキャプチャの開始
    let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unsupported channel type"),
        Err(e) => panic!("Failed to create datalink channel: {}", e),
    };

    println!("Listening on interface: {}", interface_name);

    // 受信ループ
    loop {
        match rx.next() {
            Ok(packet) => {
                if let Some(eth_packet) = EthernetPacket::new(packet) {
                    println!("Captured packet: {:?}", eth_packet);
                }
            }
            Err(e) => {
                eprintln!("Failed to receive packet: {}", e);
            }
        }
    }
}
