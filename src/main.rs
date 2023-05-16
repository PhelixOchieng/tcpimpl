use std::io;

fn main() -> io::Result<()> {
    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;
    let mut buf = [0u8; 1054];

    loop {
        let n_bytes = nic.recv(&mut buf)?;

        let _eth_flags = u16::from_be_bytes([buf[0], buf[1]]);
        let eth_proto = u16::from_be_bytes([buf[2], buf[3]]);

        if eth_proto != 0x0800 {
            // Ignore non IPV4 packets
            continue;
        }

        let ip_packet = match etherparse::Ipv4HeaderSlice::from_slice(&buf[4..n_bytes]) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("Ignoring invalid IPV4 ip_:\n{:?}", e);
                continue;
            }
        };

        let src_addr = ip_packet.source_addr();
        let dst_addr = ip_packet.destination_addr();
        let proto = ip_packet.protocol();
        if proto != 0x06 {
            // Ignore non TCP connections
            continue;
        }

        let tcp_slice = &buf[4 + ip_packet.slice().len()..];
        let tcp_packet = match etherparse::TcpHeaderSlice::from_slice(&tcp_slice) {
            Ok(p) => p,
            Err(e) => {
                eprintln!("Ignoring invalid TCP packet:\n{:?}", e);
                continue;
            }
        };

        println!(
            "{} -> {} {}b TCP {} -> {}",
            src_addr,
            dst_addr,
            ip_packet.total_len(),
            tcp_packet.source_port(),
            tcp_packet.destination_port(),
        );
    }
}
