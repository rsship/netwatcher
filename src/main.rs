mod customerror;
mod getifaddrs;
use core::time;
use customerror::CustomError;
use rand::Rng;
use std::mem;
use std::net::{IpAddr, Ipv4Addr};
use std::net::{SocketAddr, UdpSocket};
use std::thread;

fn main2() {
    let socket = UdpSocket::bind("0.0.0.0:0").expect("could not get ip");
    let socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)), 53);
    loop {
        println!("checking network status...");
        match socket.connect(socket_addr) {
            Ok(()) => {
                println!("you're already connected to internet");
            }
            Err(err) => {
                if err
                    .raw_os_error()
                    .expect("could not cast into raw os error type")
                    == 51
                {
                    todo!("not implemented yet");
                } else {
                    println!("you've got another problem pal");
                }
            }
        }
        thread::sleep(time::Duration::from_secs(60 * 10))
    }
}

fn main() {
    let mut addr = mem::MaybeUninit::<*mut libc::ifaddrs>::uninit();
    match unsafe { libc::getifaddrs(addr.as_mut_ptr()) } {
        0 => {
            let x = unsafe { addr.assume_init()};
            println!("x {:?}", x);
            Ok(())
        },
        getifaddrs_result => Err::<(), _>(CustomError::GetIfAddrsError(
            String::from("getifaddrs"),
            getifaddrs_result,
        )),
    }
    .expect("not an error");
}
fn random_mac_address(local_admin: bool) -> Result<String, ()> {
    let mut rng = rand::thread_rng();
    let random = rng.gen_range(0..9);
    let vendors = Vec::from([
        [0x00, 0x05, 0x69], // VMWware Macs
        [0x00, 0x50, 0x56], // vMWare  Macs
        [0x00, 0x0C, 0x29], // VMWare  Macs
        [0x00, 0x16, 0x3E], // Xen VMs
        [0x00, 0x03, 0xFF], // Microsoft Hyper-V
        [0x00, 0x1C, 0x42], // Parallels
        [0x00, 0x0F, 0x4B], // Virtual Iron 4
        [0x08, 0x00, 0x27], // Sun Virtual Box
    ]);
    let vendor = vendors[random];
    let mut mac = Vec::from([
        vendor[0],
        vendor[1],
        vendor[2],
        rng.gen_range(0x00..0x7F),
        rng.gen_range(0x00..0xFF),
        rng.gen_range(0x00..0xFF),
    ]);
    if local_admin {
        mac[0] |= 2;
    }

    let random_mac = mac
        .iter()
        .map(|x| format!("{:02X}", x))
        .collect::<Vec<_>>()
        .join(":");
    Ok(random_mac)
}
