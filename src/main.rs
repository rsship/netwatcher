mod customerror;
mod getifaddrs;
mod random;
use getifaddrs::IfAddr;
use libc::{
    c_int, c_ulong, ioctl, sockaddr, strlen, AF_INET, IPPROTO_UDP, SIOCSIFADDR, SOCK_DGRAM,
};
use std::io::Error;
use std::io::Write;
use std::net::UdpSocket;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::slice::from_raw_parts;
use std::thread;
use std::time;
use random as mac_random;

// sudo ip link set dev wlx90de80910f38 down
// sudo macchanger -r wlx90de80910f38
// sudo ip link set dev wlx90de80910f38 up

fn set_address_from_name(
    sock: c_int,
    dev: &str,
    ioctl_num: c_ulong,
    value: c_int,
) -> Result<(), Error> {
    let mut req = IfReqAddr {
        name: [0; 16],
        value,
    };
    req.name.as_mut().write_all(dev.as_bytes())?;
    match unsafe { ioctl(sock, ioctl_num, &mut req) } {
        -1 => Err(Error::last_os_error()),
        _ => Ok(()),
    }
}
#[derive(Debug, Clone)]
struct IfReqAddr {
    name: [u8; 16],
    value: c_int,
}

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

fn get_interfaces() -> anyhow::Result<Vec<String>> {
    let if_addr_iter = IfAddr::new()?;
    let mut interfaces = Vec::new();

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
                interfaces.push(name);
            }
            _ => continue,
        }
    }

    Ok(interfaces)
}

fn get_socket() -> anyhow::Result<c_int> {
    let res = unsafe { libc::socket(AF_INET, SOCK_DGRAM, IPPROTO_UDP) };
    match res {
        -1 => Err(Error::last_os_error().into()),
        sock => Ok(sock),
    }
}

#[repr(C)]
struct ifreq_addr {
    name: [u8; 16],
    value: sockaddr,
}

fn main() -> anyhow::Result<()> {
    let interfaces = get_interfaces()?;
    let sock = get_socket()?;
    for intf in interfaces {
        let ioctl_num = SIOCSIFADDR;
        let random_addr = mac_random::random_mac_address()?;
        let mut req = ifreq_addr {
            name: [0; 16],
            value: random_addr,
        };

        req.name.as_mut().write_all(intf.as_bytes())?;
        match unsafe { ioctl(sock, ioctl_num, &mut req) } {
            _ => {}
        }
    }
    Ok(())
}
