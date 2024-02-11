use rand::Rng;

pub fn random_mac_address() -> anyhow::Result<String> {
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
