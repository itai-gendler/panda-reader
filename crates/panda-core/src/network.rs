//! Shared destination checks for network requests initiated by article content.

use serde::{Deserialize, Serialize};
use std::net::{IpAddr, SocketAddr, ToSocketAddrs};
use url::{Host, Url};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArticleNetworkMode {
    /// Use available system/environment proxies while preserving strict DNS checks.
    #[default]
    System,
    /// Bypass configured proxies and connect directly to a validated public IP.
    Direct,
    /// Send requests through a configured HTTP or SOCKS5h proxy.
    ExplicitProxy,
    /// Allow configured synthetic DNS ranges for a trusted transparent proxy/TUN.
    TrustedTransparent,
}

impl ArticleNetworkMode {
    pub const ALL: [Self; 4] = [
        Self::System,
        Self::Direct,
        Self::ExplicitProxy,
        Self::TrustedTransparent,
    ];

    pub fn index(self) -> usize {
        Self::ALL.iter().position(|mode| *mode == self).unwrap_or(0)
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::System => "System proxy",
            Self::Direct => "Direct connection",
            Self::ExplicitProxy => "Explicit proxy",
            Self::TrustedTransparent => "Trusted transparent proxy",
        }
    }
}

/// Validate URL properties that do not depend on the selected network route.
pub fn validate_http_destination(url: &Url) -> Result<(), &'static str> {
    if !matches!(url.scheme(), "http" | "https")
        || !url.username().is_empty()
        || url.password().is_some()
    {
        return Err("destination is not a safe HTTP URL");
    }
    let host = url.host().ok_or("destination URL has no host")?;
    if let Host::Domain(domain) = host {
        let normalized = domain.trim_end_matches('.').to_ascii_lowercase();
        if normalized == "localhost"
            || normalized.ends_with(".localhost")
            || normalized.ends_with(".local")
            || normalized == "metadata.google.internal"
        {
            return Err("destination points to a local or metadata host");
        }
    }
    if let Some(address) = host_ip(host)
        && !is_public_ip(address)
    {
        return Err("destination IP is not publicly routable");
    }
    Ok(())
}

/// Resolve a hostname once and reject the whole answer if any address is unsafe.
/// Callers should pin the returned addresses on the client used for the request.
pub fn resolve_public_addresses(host: &str, port: u16) -> anyhow::Result<Vec<SocketAddr>> {
    let addresses: Vec<_> = (host, port).to_socket_addrs()?.collect();
    if addresses.is_empty() || addresses.iter().any(|address| !is_public_ip(address.ip())) {
        anyhow::bail!("destination hostname resolved to a non-public network address");
    }
    Ok(addresses)
}

pub fn is_public_ip(address: IpAddr) -> bool {
    match address {
        IpAddr::V4(address) => {
            let octets = address.octets();
            let first_two = u16::from(octets[0]) << 8 | u16::from(octets[1]);
            !(address.is_private()
                || address.is_loopback()
                || address.is_link_local()
                || address.is_broadcast()
                || address.is_unspecified()
                || address.is_multicast()
                || octets[0] == 0
                || octets[0] >= 240
                || (0x6440..=0x647f).contains(&first_two) // Shared address space, 100.64.0.0/10.
                || (octets[0] == 192 && octets[1] == 0 && octets[2] == 0)
                || (octets[0] == 192 && octets[1] == 0 && octets[2] == 2)
                || (octets[0] == 192 && octets[1] == 88 && octets[2] == 99)
                || (octets[0] == 198 && (18..=19).contains(&octets[1]))
                || (octets[0] == 198 && octets[1] == 51 && octets[2] == 100)
                || (octets[0] == 203 && octets[1] == 0 && octets[2] == 113))
        }
        IpAddr::V6(address) => {
            let segments = address.segments();
            let global_unicast = segments[0] & 0xe000 == 0x2000;
            global_unicast
                && !(address.is_loopback()
                    || address.is_unspecified()
                    || address.is_multicast()
                    || (segments[0] == 0x2001
                        && (segments[1] & 0xfe00 == 0 || segments[1] == 0x0db8))
                    || segments[0] == 0x2002
                    || address
                        .to_ipv4_mapped()
                        .is_some_and(|ipv4| !is_public_ip(IpAddr::V4(ipv4))))
        }
    }
}

fn host_ip(host: Host<&str>) -> Option<IpAddr> {
    match host {
        Host::Domain(_) => None,
        Host::Ipv4(address) => Some(IpAddr::V4(address)),
        Host::Ipv6(address) => Some(IpAddr::V6(address)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn destination_policy_rejects_local_and_special_addresses() {
        for address in [
            "127.0.0.1",
            "10.0.0.1",
            "169.254.1.1",
            "100.64.0.1",
            "192.0.2.1",
            "::1",
            "::ffff:127.0.0.1",
            "fc00::1",
            "fe80::1",
            "2001:db8::1",
        ] {
            assert!(!is_public_ip(address.parse().unwrap()), "{address}");
        }
        for address in ["1.1.1.1", "2606:4700:4700::1111"] {
            assert!(is_public_ip(address.parse().unwrap()), "{address}");
        }
    }

    #[test]
    fn destination_url_checks_are_independent_of_dns_routing() {
        assert!(
            validate_http_destination(&Url::parse("https://example.org/article").unwrap()).is_ok()
        );
        for url in [
            "file:///etc/passwd",
            "http://user:pass@example.org/",
            "http://localhost/",
            "http://127.0.0.1/",
            "http://metadata.google.internal/",
        ] {
            assert!(
                validate_http_destination(&Url::parse(url).unwrap()).is_err(),
                "{url}"
            );
        }
    }

    #[test]
    fn resolved_localhost_is_rejected_before_a_direct_request() {
        assert!(resolve_public_addresses("localhost", 443).is_err());
    }
}
