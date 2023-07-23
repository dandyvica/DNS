use windows::Win32::{
    Foundation::{ERROR_BUFFER_OVERFLOW, ERROR_SUCCESS},
    NetworkManagement::IpHelper::{
        GetAdaptersAddresses, GAA_FLAG_INCLUDE_PREFIX, IP_ADAPTER_ADDRESSES_LH,
    },
    Networking::WinSock::AF_INET,
};

fn servers() -> (u32, Vec<IpAddr>) {
    let mut v = Vec::new();

    // first call
    let family = AF_INET.0 as u32;
    let mut buflen = 0u32;
    let mut rc =
        unsafe { GetAdaptersAddresses(family, GAA_FLAG_INCLUDE_PREFIX, None, None, &mut buflen) };

    // second with the actual buffer size large enough to hold data
    if rc == ERROR_BUFFER_OVERFLOW.0 {
        let mut addr = vec![0u8; buflen as usize];
        let ptr = addr.as_mut_ptr() as *mut IP_ADAPTER_ADDRESSES_LH;

        rc = unsafe {
            GetAdaptersAddresses(
                family,
                GAA_FLAG_INCLUDE_PREFIX,
                None,
                Some(ptr),
                &mut buflen,
            )
        };

        // second with the actual buffer size large enough to hold data
        if rc == ERROR_SUCCESS.0 {
            // loop through adapters and grab DNS addresses
            let mut p = ptr;

            while !p.is_null() {
                unsafe {
                    let mut p_dns = (*p).FirstDnsServerAddress;

                    // loop through DNS addresses for this adapter
                    while !p_dns.is_null() {
                        let sockaddr = (*p_dns).Address.lpSockaddr;
                        let dns_addr = from_sockaddr((*sockaddr).sa_data, (*sockaddr).sa_family.0);
                        v.push(dns_addr);

                        p_dns = (*p_dns).Next;
                    }

                    p = (*p).Next;
                }
            }
            (rc, v)
        } else {
            (rc, v)
        }
    } else {
        (rc, v)
    }
}

// utility function which is used to build an IpAddr from an array used in Windows OS
pub fn from_sockaddr(addr: [u8; 14], family: u16) -> IpAddr {
    // this is only valid for INET4 or 6 family
    match family {
        2 => {
            // ip v4 addresses reported by GetAdaptersAddresses() API are like: [0, 0, 8, 8, 8, 8, 0, 0, 0, 0, 0, 0, 0, 0] (for 8.8.8.8)
            let data: [u8; 4] = addr[2..6].try_into().unwrap();
            IpAddr::from(data)
        }
        6 => {
            unimplemented!("not IPV6 yet")
        }
        _ => panic!("unexpected AF family"),
    }
}
