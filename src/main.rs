use std::io;

fn main() -> io::Result<()> {
    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;
    let mut buf = [0u8; 1054];

    loop {
        let nbytes = nic.recv(&mut buf)?;
        println!("read {} bytes: {:x?}", nbytes, &buf[..nbytes]);
    }
}
