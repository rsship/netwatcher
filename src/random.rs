use libc::sockaddr;
use rand::Rng;

#[derive(Debug)]
pub struct hardware_address {
     data: [u8;6]
}

impl hardware_address {
    pub fn new () -> Self {
        Self { data: [0u8;6]}
    }
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        let random_int = rng.gen_range(0..8);

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

        let random_vendor = vendors[random_int];
        Self {
            data: [
                random_vendor[0], 
                random_vendor[1],
                random_vendor[2],
                rng.gen_range(0x00..0x7F),
                rng.gen_range(0x00..0xFF),
                rng.gen_range(0x00..0xFF),
            ]
        }
    }
}

impl From<sockaddr> for hardware_address {
    fn from(sock: sockaddr) -> Self { 
        let mut out = Self::new(); 
        for (n, x) in sock.sa_data[0..6].iter().enumerate() {
            out.data[n] = *x as u8;
        }
        out
    }
}

impl Into<[i8;14]> for hardware_address {
    fn into(self) -> [i8; 14] { 
        let mut out: [i8;14] = [0;14];
        for (n, x) in self.data.iter().enumerate() {
            out[n] =  *x as i8;
        }
        out
    }
}


