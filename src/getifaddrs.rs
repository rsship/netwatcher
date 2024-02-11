use super::customerror::Error;
use std::mem;

pub struct IfAddr {
    base: *mut libc::ifaddrs,
    next: *mut libc::ifaddrs,
}

impl Iterator for IfAddr {
    type Item = libc::ifaddrs;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        match unsafe { self.next.as_ref() } {
            Some(ifaddrs) => {
                self.next = ifaddrs.ifa_next;
                Some(*ifaddrs)
            }
            None => None,
        }
    }
}

impl Drop for IfAddr {
    fn drop(&mut self) {
        unsafe { libc::freeifaddrs(self.base) }
    }
}

impl IfAddr {
    pub fn new() -> anyhow::Result<Self> {
        let mut addr = mem::MaybeUninit::<*mut libc::ifaddrs>::uninit();
        match unsafe { libc::getifaddrs(addr.as_mut_ptr()) } {
            0 => Ok(IfAddr {
                base: unsafe { addr.assume_init() },
                next: unsafe { addr.assume_init() },
            }),
            getifaddrs_result => {
                Err(Error::GetIfAddrsError("getifaddrs".to_string(), getifaddrs_result).into())
            }
        }
    }
}

// AF_PACKET => {
//     continue;
// }
// AF_INET6 => {
//     // NOTE: IPV6, currently not interested in
//     // let socket_addr = net_if_addr as *mut sockaddr_in6;
//     // let internet_addr = unsafe { (*socket_addr).sin6_addr };
//     // let name = name_from_if(&net_if)?;
//     // let addr = ipv6_from_in6_addr(&internet_addr)?;
//     // let i6net_inter = NetInterface::new(name, addr.to_string());
//     // println!("i6net {:?}", i6net_inter);
//     continue;
// }
