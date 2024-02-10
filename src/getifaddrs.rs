use std::mem;

use super::customerror::Error;

pub struct IfAddrIterator {
    base: *mut libc::ifaddrs,
    next: *mut libc::ifaddrs,
}

impl Iterator for IfAddrIterator {
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

impl Drop for IfAddrIterator {
    fn drop(&mut self) {
        unsafe { libc::freeifaddrs(self.base) }
    }
}

pub fn getifaddrs() -> anyhow::Result<IfAddrIterator> {
    let mut addr = mem::MaybeUninit::<*mut libc::ifaddrs>::uninit();
    match unsafe { libc::getifaddrs(addr.as_mut_ptr()) } {
        0 => Ok(IfAddrIterator {
            base: unsafe { addr.assume_init() },
            next: unsafe { addr.assume_init() },
        }),
        getifaddrs_result => {
            Err(Error::GetIfAddrsError("getifaddres".to_string(), getifaddrs_result).into())
        }
    }
}
