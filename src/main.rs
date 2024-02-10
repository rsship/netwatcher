mod customerror;
mod getifaddrs;
use crate::getifaddrs::getifaddrs;
use core::time;
use libc::{sockaddr_in, strlen, AF_INET, AF_INET6};
use rand::Rng;
use std::net::{IpAddr, Ipv4Addr};
use std::net::{SocketAddr, UdpSocket};
use std::slice::from_raw_parts;
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

// let socket_addr = netifa_addr as *mut sockaddr_in;
// let internet_address = unsafe { (*socket_addr).sin_addr };
// let name = make_netifa_name(&netifa)?;
// let index = netifa_index(&netifa);
// let netmask = make_ipv4_netmask(&netifa);
// let addr = ipv4_from_in_addr(&internet_address)?;
// let broadcast = make_ipv4_broadcast_addr(&netifa)?;
// NetworkInterface::new_afinet(name.as_str(), addr, netmask, broadcast, index)

struct NetInterface {
    name: String,
    addr: String,
    mac_addr: String,
    index: u32,
}

fn main() -> anyhow::Result<()> {
    for net_if in getifaddrs()? {
        let net_if_addr = net_if.ifa_addr;
        let net_if_family = if net_if_addr.is_null() {
            continue;
        } else {
            unsafe { (*net_if_addr).sa_family as i32 }
        };

        match net_if_family {
            AF_INET => {
                let socket_addr = net_if_addr as *mut sockaddr_in;
                let internet_addr = unsafe { (*socket_addr).sin_addr };
                let addr = Ipv4Addr::from(internet_addr.s_addr).to_string();
                let name = get_net_if_name(&net_if)?;
                let netmask = "netmask";
                let broadcast = "broadcast";
                let mac_addr = "mac_adr";
                let index = 0;

                return NetInterface {
                    name, 
                    addr, 
                    mac_addr: mac_addr.to_string(),
                    index,
                }
            }
            AF_INET6 => {
                println!("not impplemented yet AF_INET6");
            }
            _ => {}
        }
    }
    Ok(())
}

fn get_net_if_name(net_if: &libc::ifaddrs) -> anyhow::Result<String> {
    let data = net_if.ifa_name as *const libc::c_char;
    let len = unsafe { strlen(data) };
    let byte_slice = unsafe { from_raw_parts(data as *const u8, len) };
    Ok(String::from_utf8(byte_slice.to_vec())?)
}

fn random_mac_address() -> Result<String, ()> {
    let mut rng = rand::thread_rng();
    let random = rng.gen_range(0..9);
    let vendors = Vec::from([
        [0x00, 0x05, 0x69], //  VMWware Macs
        [0x00, 0x50, 0x56], //  vMWare  Macs
        [0x00, 0x0C, 0x29], //  VMWare  Macs
        [0x00, 0x16, 0x3E], //  Xen VMs
        [0x00, 0x03, 0xFF], //  Microsoft Hyper-V
        [0x00, 0x1C, 0x42], //  Parallels
        [0x00, 0x0F, 0x4B], //  Virtual Iron 4
        [0x08, 0x00, 0x27], //  Sun Virtual Box
    ]);
    let vendor = vendors[random];
    let mac = Vec::from([
        vendor[0],
        vendor[1],
        vendor[2],
        rng.gen_range(0x00..0x7F),
        rng.gen_range(0x00..0xFF),
        rng.gen_range(0x00..0xFF),
    ]);

    let random_mac = mac
        .iter()
        .map(|x| format!("{:02X}", x))
        .collect::<Vec<_>>()
        .join(":");
    Ok(random_mac)
}
