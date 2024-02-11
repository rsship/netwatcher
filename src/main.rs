mod customerror;
mod getifaddrs;
mod random;
use getifaddrs::IfAddr;
use libc::{
    c_int, c_ulong, ioctl, sockaddr, sockaddr_in, strlen, AF_INET, IPPROTO_UDP, SOCK_DGRAM,
};
use mac_random::hardware_address;
use random as mac_random;
use std::io::Error;
use std::io::Write;
use std::net::UdpSocket;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::slice::from_raw_parts;
use std::thread;
use std::time;

// sudo ip link set dev wlx90de80910f38 down
// sudo macchanger -r wlx90de80910f38
// sudo ip link set dev wlx90de80910f38 up
//
const SIOCSIFHWADDR: c_ulong = 0x8924;
const SIOCGIFADDR: c_ulong = 0x8915;

pub fn name_from_if(net_if: &libc::ifaddrs) -> anyhow::Result<String> {
    let data = net_if.ifa_name as *const libc::c_char;
    let len = unsafe { strlen(data) };
    let byte_slice = unsafe { from_raw_parts(data as *const u8, len) };
    Ok(String::from_utf8(byte_slice.to_vec())?)
}

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

#[repr(C)]
struct ifreq_address {
    name: [u8; 16],
    value: sockaddr,
}

// set_address_from_name(sock, dev, SIOCSIFHWADDR, address)
// fn set_address_from_name(sock: c_int, dev: &str, ioctl_num: c_ulong, value: sockaddr) -> Result<(), Error> {
//     let mut req = ifreq_address {
//         name: [0; 16],
//         value: value,
//     };
//     try!(req.name.as_mut().write_all(dev.as_bytes()));
//     match unsafe { ioctl(sock, ioctl_num, &mut req) } {
//         -1 => Err(Error::last_os_error()),
//         _ => Ok(())
//     }
// }

fn main() -> anyhow::Result<()> {
    let if_addr_iter = IfAddr::new()?;
    for net_if in if_addr_iter {
        let net_if_addr = net_if.ifa_addr;
        let net_if_family = if net_if_addr.is_null() {
            continue;
        } else {
            unsafe { (*net_if_addr).sa_family as i32 }
        };

        match net_if_family {
            AF_INET => {
                let name = name_from_if(&net_if)?;
                // NOTE: selecting only one of interfaces for testing purposes
                let raw_sock_addr = net_if_addr;
                let random_mac = hardware_address::random();
                unsafe { (*raw_sock_addr).sa_data = random_mac.into() }

                let sock_addr = unsafe { (*raw_sock_addr) as sockaddr };
                let mut req = ifreq_address {
                    name: [0; 16],
                    value: sock_addr,
                };
                req.name.as_mut().write_all(name.as_bytes())?;
                let res = match unsafe { libc::socket(AF_INET, SOCK_DGRAM, IPPROTO_UDP) } {
                    -1 => Err(Error::last_os_error()),
                    sock => Ok(sock),
                }?;

                if unsafe { ioctl(res, SIOCSIFHWADDR, &mut req) } < 0 {
                    return Err(Error::last_os_error().into());
                }
            }
            _ => continue,
        }
    }

    Ok(())
}
