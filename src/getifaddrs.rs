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
