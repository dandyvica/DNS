mod error;
mod srv;

#[cfg(test)]
mod tests {
    use std::net::IpAddr;
    use std::str::FromStr;

    use crate::error::Error;
    use crate::srv::DNSSrv;

    #[test]
    fn servers() {
        // non existing file
        let servers = DNSSrv::servers(Some("/tmp/foo"));
        assert!(servers.is_err());
        assert!(matches!(servers.unwrap_err(), Error::Io(_)));

        // first list is a list with well constructed ip addresses
        let servers = DNSSrv::servers(Some("test/data/resolv.ok"));
        assert!(servers.is_ok());
        let s = servers.unwrap();
        assert_eq!(s.len(), 4);
        assert_eq!(s[0], IpAddr::from_str("8.8.4.4").unwrap());
        assert_eq!(s[1], IpAddr::from_str("8.8.8.8").unwrap());
        assert_eq!(s[2], IpAddr::from_str("2001:4860:4860::8888").unwrap());
        assert_eq!(s[3], IpAddr::from_str("2001:4860:4860::8844").unwrap());

        // DNS list where ip addresses are malformed
        let servers = DNSSrv::servers(Some("test/data/resolv.bad"));
        assert!(servers.is_err());
        assert!(matches!(servers.unwrap_err(), Error::AddrParseError(_)));
    }
}
