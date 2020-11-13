extern crate net2;

use std::net::{TcpStream, IpAddr, Ipv4Addr, Ipv6Addr};
use std::io::prelude::*;
use std::thread;

use net2::TcpBuilder;

macro_rules! t {
    ($e:expr) => (match $e {
        Ok(e) => e,
        Err(e) => panic!("{} failed with: {}", stringify!($e), e),
    })
}

#[test]
fn smoke_build_listener() {
    let b = t!(TcpBuilder::new_v4());
    t!(b.bind("127.0.0.1:0"));

    let addr = t!(b.local_addr());
    assert_eq!(addr.ip(), IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));

    let listener = t!(b.listen(200));

    let t = thread::spawn(move || {
        let mut s = t!(listener.accept()).0;
        let mut b = [0; 4];
        t!(s.read(&mut b));
        assert_eq!(b, [1, 2, 3, 0]);
    });

    let mut stream = t!(TcpStream::connect(&addr));
    t!(stream.write(&[1,2,3]));
    t.join().unwrap();
}

#[test]
fn smoke_build_listener_v6() {
    let b = t!(TcpBuilder::new_v6());
    t!(b.bind("::1:0"));

    let addr = t!(b.local_addr());
    assert_eq!(addr.ip(), IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)));
}

#[test]
fn print_sockaddr() {
    env_logger::init();
    use net2::addr2raw;
    use std::net::SocketAddr;

    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(6, 7, 8, 9)), 0x0304);

    let (raw_addr, len) = addr2raw(&addr);
    let slice = unsafe { std::slice::from_raw_parts(raw_addr as *const u8, len as usize) };
    eprintln!("HALLÅ {}, {:?}", len, slice);

    let addr = SocketAddr::new(IpAddr::V6(Ipv6Addr::new(
        0x0304,
        0x0506,
        0x0708,
        0x090a,
        0x0b0c,
        0x0d0e,
        0x0f10,
        0x1112)),
    0xffa0);

    let (raw_addr, len) = addr2raw(&addr);
    let slice = unsafe { std::slice::from_raw_parts(raw_addr as *const u8, len as usize) };
    eprintln!("HALLÅ {} {:?}", len, slice);
}