use anyhow::Context as _;
use reqwest::Client;
use std::time::Duration;
use url::Url;

const MAX_ARTICLE_PAGE_BYTES: usize = 20 * 1024 * 1024;
const MAX_ARTICLE_REDIRECTS: usize = 10;
const BROWSER_UA: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/137.0.0.0 Safari/537.36";

pub(super) async fn fetch_html(url: &str, referer: Option<&str>) -> anyhow::Result<String> {
    let response = fetch_public_destination(url, referer).await?;
    let content_type = response
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default()
        .to_owned();
    let bytes = read_response_limited(response, MAX_ARTICLE_PAGE_BYTES, "article page").await?;
    let charset = content_type
        .split(';')
        .find_map(|part| {
            let (name, value) = part.trim().split_once('=')?;
            name.trim()
                .eq_ignore_ascii_case("charset")
                .then_some(value.trim())
        })
        .unwrap_or("utf-8")
        .trim_matches([' ', '\'', '"']);
    let encoding =
        encoding_rs::Encoding::for_label(charset.as_bytes()).unwrap_or(encoding_rs::UTF_8);
    Ok(encoding.decode(&bytes).0.into_owned())
}

pub(super) async fn fetch_public_destination(
    url: &str,
    referer: Option<&str>,
) -> anyhow::Result<reqwest::Response> {
    let mut current_url = Url::parse(url)?;
    for redirect_count in 0..=MAX_ARTICLE_REDIRECTS {
        let pinned_host = pin_public_destination(&current_url).await?;
        let mut client = Client::builder()
            .timeout(Duration::from_secs(25))
            .redirect(reqwest::redirect::Policy::none());
        if let Some((host, addresses)) = pinned_host {
            client = client.resolve_to_addrs(&host, &addresses);
        }
        let client = client.build()?;
        let mut request = client
            .get(current_url.clone())
            .header(reqwest::header::USER_AGENT, BROWSER_UA)
            .header(
                reqwest::header::ACCEPT,
                "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8",
            )
            .header(
                reqwest::header::ACCEPT_LANGUAGE,
                "zh-CN,zh;q=0.9,en-US;q=0.8,en;q=0.7",
            );
        if let Some(referer) = referer {
            request = request.header(reqwest::header::REFERER, referer);
        }
        let response = request.send().await?;
        let status = response.status();
        if status == reqwest::StatusCode::UNAUTHORIZED || status == reqwest::StatusCode::FORBIDDEN {
            anyhow::bail!(
                "Publisher blocked automated article downloads ({status}). Open the original page instead."
            );
        }
        if status.is_redirection() {
            if redirect_count == MAX_ARTICLE_REDIRECTS {
                anyhow::bail!("article page exceeded the redirect limit");
            }
            let location = response
                .headers()
                .get(reqwest::header::LOCATION)
                .and_then(|value| value.to_str().ok())
                .context("article redirect has no valid Location header")?;
            current_url = article_redirect_url(&current_url, location)?;
            continue;
        }
        return Ok(response.error_for_status()?);
    }
    anyhow::bail!("article page exceeded the redirect limit")
}

pub(super) fn article_redirect_url(current: &Url, location: &str) -> anyhow::Result<Url> {
    let next = current.join(location)?;
    if next.scheme() != "http" && next.scheme() != "https" {
        anyhow::bail!("article redirect used an unsupported URL scheme");
    }
    if current.scheme() == "https" && next.scheme() == "http" {
        anyhow::bail!("article redirect attempted to downgrade HTTPS to HTTP");
    }
    Ok(next)
}

pub(super) async fn pin_public_destination(
    url: &Url,
) -> anyhow::Result<Option<(String, Vec<std::net::SocketAddr>)>> {
    use std::net::IpAddr;

    if !matches!(url.scheme(), "http" | "https")
        || !url.username().is_empty()
        || url.password().is_some()
    {
        anyhow::bail!("article URL is not a safe HTTP destination");
    }
    let host = url.host().context("article URL has no host")?;
    let port = url
        .port_or_known_default()
        .context("article URL has no port")?;
    match host {
        url::Host::Domain(domain) => {
            let domain = domain.to_owned();
            let lookup_domain = domain.clone();
            let addresses: Vec<_> = tokio::task::spawn_blocking(move || {
                use std::net::ToSocketAddrs as _;
                (lookup_domain.as_str(), port)
                    .to_socket_addrs()
                    .map(|addresses| addresses.collect())
            })
            .await??;
            if addresses.is_empty() || addresses.iter().any(|address| !is_public_ip(address.ip())) {
                anyhow::bail!("article URL resolves to a non-public network address");
            }
            Ok(Some((domain.to_owned(), addresses)))
        }
        url::Host::Ipv4(address) => {
            if !is_public_ip(IpAddr::V4(address)) {
                anyhow::bail!("article URL resolves to a non-public network address");
            }
            Ok(None)
        }
        url::Host::Ipv6(address) => {
            if !is_public_ip(IpAddr::V6(address)) {
                anyhow::bail!("article URL resolves to a non-public network address");
            }
            Ok(None)
        }
    }
}

pub(super) fn is_public_ip(address: std::net::IpAddr) -> bool {
    use std::net::IpAddr;

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
                    || segments[0] == 0x2002)
        }
    }
}

pub(super) async fn read_response_limited(
    mut response: reqwest::Response,
    limit: usize,
    label: &str,
) -> anyhow::Result<Vec<u8>> {
    if response
        .content_length()
        .is_some_and(|length| length > limit as u64)
    {
        anyhow::bail!("{label} response exceeds the {limit}-byte limit");
    }
    let mut bytes =
        Vec::with_capacity(response.content_length().unwrap_or(0).min(limit as u64) as usize);
    while let Some(chunk) = response.chunk().await? {
        if bytes.len().saturating_add(chunk.len()) > limit {
            anyhow::bail!("{label} response exceeds the {limit}-byte limit");
        }
        bytes.extend_from_slice(&chunk);
    }
    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn article_fetch_only_allows_public_ip_destinations() {
        for address in [
            "127.0.0.1",
            "10.0.0.1",
            "169.254.1.1",
            "100.64.0.1",
            "192.0.2.1",
            "::1",
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

    #[tokio::test]
    async fn article_fetch_rejects_loopback_and_pins_public_dns_results() {
        assert!(
            pin_public_destination(&Url::parse("http://127.0.0.1/").unwrap())
                .await
                .is_err()
        );
        assert!(
            pin_public_destination(&Url::parse("http://[::1]/").unwrap())
                .await
                .is_err()
        );
        assert!(
            pin_public_destination(&Url::parse("http://localhost/").unwrap())
                .await
                .is_err()
        );
    }

    #[tokio::test]
    async fn article_redirect_destinations_use_the_same_public_address_check() {
        let current = Url::parse("http://example.com/story").unwrap();
        let redirected = article_redirect_url(&current, "http://127.0.0.1/admin").unwrap();
        assert!(pin_public_destination(&redirected).await.is_err());
        assert!(
            article_redirect_url(
                &Url::parse("https://example.com").unwrap(),
                "http://example.com"
            )
            .is_err()
        );
        assert!(article_redirect_url(&current, "file:///etc/passwd").is_err());
    }
}
