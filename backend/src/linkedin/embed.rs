//! Parse and validate a LinkedIn "Embed this post" snippet or URL.
//!
//! Security rule: we never store what the user pasted. We pull out one value
//! (the URL), check every part of it, and then *rebuild* a canonical URL from
//! the pieces we validated. Anything else in the input is thrown away.

use std::sync::LazyLock;

use regex::Regex;
use url::Url;

const ALLOWED_HOST: &str = "www.linkedin.com";
/// Real snippets are ~300 bytes. Anything far larger is not a snippet.
const MAX_INPUT_LEN: usize = 4096;

/// Finds the `src="..."` value inside a pasted `<iframe ...>` snippet.
///
/// ```text
/// (?i)            case-insensitive flag: matches src, SRC, Src
/// (?:^|\s)        non-capturing group: start of text OR one whitespace char.
///                 Stops `data-src=` from matching (the char before `src` is `-`).
/// src             the literal letters s, r, c
/// \s*=\s*         `=` with optional whitespace (`*` = zero or more) around it
/// ["']            character class: one double quote or one single quote
/// ([^"']+)        capture group 1: one or more (`+`) chars that are NOT quotes
///                 (`[^...]` = negated class). This is the URL we want.
/// ["']            the closing quote
/// ```
static SRC_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?i)(?:^|\s)src\s*=\s*["']([^"']+)["']"#).expect("SRC_RE is a valid regex")
});

/// Validates the *path* part of the URL and captures the URN.
///
/// ```text
/// ^                         anchor: match must start at the beginning of the path
/// /embed/feed/update/       literal text (`/` needs no escaping in Rust regex)
/// (                         capture group 1 = the URN
///   urn:li:
///   (?:share|ugcPost|activity)   alternation: exactly one of these three words
///   :
///   [0-9]{1,25}             1 to 25 ASCII digits. NOT `\d`: in Rust `\d` also
///                           matches non-ASCII digits such as Arabic-Indic ones.
/// )
/// $                         anchor: match must end at the end of the path,
///                           so `/extra` or a trailing `/` is rejected
/// ```
///
/// No `(?i)` here on purpose: `ugcPost` must be spelled exactly.
static PATH_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^/embed/feed/update/(urn:li:(?:share|ugcPost|activity):[0-9]{1,25})$")
        .expect("PATH_RE is a valid regex")
});

#[derive(Debug, PartialEq)]
pub struct ParsedEmbed {
    /// Normalized: `https://www.linkedin.com/embed/feed/update/urn:li:share:123`
    pub embed_url: String,
    /// e.g. `urn:li:share:123`, used for de-duplication.
    pub urn: String,
}

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum EmbedError {
    #[error("no src attribute or URL found")]
    NoUrl,
    #[error("not a valid URL")]
    InvalidUrl,
    #[error("URL must use https")]
    NotHttps,
    #[error("URL must be on www.linkedin.com")]
    WrongHost,
    #[error("URL is not a LinkedIn post embed")]
    WrongPath,
}

/// Accepts either a full `<iframe ...>` snippet or a bare embed URL.
pub fn parse_embed(input: &str) -> Result<ParsedEmbed, EmbedError> {
    let input = input.trim();
    if input.is_empty() {
        return Err(EmbedError::NoUrl);
    }
    if input.len() > MAX_INPUT_LEN {
        return Err(EmbedError::InvalidUrl);
    }

    let raw_url = extract_url(input)?;
    let url = Url::parse(raw_url).map_err(|_| EmbedError::InvalidUrl)?;

    if url.scheme() != "https" {
        return Err(EmbedError::NotHttps);
    }
    // Exact comparison, never `ends_with`/`contains`: those would accept
    // `www.linkedin.com.evil.com`. Also reject `user@host` and explicit ports.
    if url.host_str() != Some(ALLOWED_HOST)
        || !url.username().is_empty()
        || url.password().is_some()
        || url.port().is_some()
    {
        return Err(EmbedError::WrongHost);
    }

    // `url.path()` excludes the query (`?...`) and fragment (`#...`), so they
    // are dropped by design.
    let caps = PATH_RE.captures(url.path()).ok_or(EmbedError::WrongPath)?;
    let urn = caps[1].to_string();

    Ok(ParsedEmbed {
        embed_url: format!("https://{ALLOWED_HOST}/embed/feed/update/{urn}"),
        urn,
    })
}

/// Snippet → the `src` value. Anything else → the input itself.
fn extract_url(input: &str) -> Result<&str, EmbedError> {
    if !input.starts_with('<') {
        return Ok(input);
    }
    SRC_RE
        .captures(input)
        .and_then(|caps| caps.get(1))
        .map(|m| m.as_str().trim())
        .ok_or(EmbedError::NoUrl)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SHARE: &str = "https://www.linkedin.com/embed/feed/update/urn:li:share:7123456789";

    fn ok(input: &str) -> ParsedEmbed {
        parse_embed(input).unwrap_or_else(|e| panic!("expected Ok for {input:?}, got {e:?}"))
    }

    fn err(input: &str) -> EmbedError {
        parse_embed(input).expect_err(input)
    }

    // ---- accepted ----

    #[test]
    fn bare_share_url() {
        let p = ok(SHARE);
        assert_eq!(p.embed_url, SHARE);
        assert_eq!(p.urn, "urn:li:share:7123456789");
    }

    #[test]
    fn ugc_post_and_activity_urls() {
        assert_eq!(
            ok("https://www.linkedin.com/embed/feed/update/urn:li:ugcPost:1").urn,
            "urn:li:ugcPost:1"
        );
        assert_eq!(
            ok("https://www.linkedin.com/embed/feed/update/urn:li:activity:2").urn,
            "urn:li:activity:2"
        );
    }

    #[test]
    fn full_snippet_with_extra_attributes() {
        let snippet = format!(
            r#"<iframe src="{SHARE}" height="600" width="504" frameborder="0" allowfullscreen="" title="Embedded post"></iframe>"#
        );
        assert_eq!(ok(&snippet).embed_url, SHARE);
    }

    #[test]
    fn snippet_with_single_quotes_uppercase_and_newlines() {
        let snippet = format!("<iframe\n  SRC = '{SHARE}'\n  height='600'></iframe>");
        assert_eq!(ok(&snippet).embed_url, SHARE);
    }

    #[test]
    fn snippet_ignores_data_src() {
        let snippet = format!(r#"<iframe data-src="https://evil.com/x" src="{SHARE}"></iframe>"#);
        assert_eq!(ok(&snippet).embed_url, SHARE);
    }

    #[test]
    fn surrounding_whitespace() {
        assert_eq!(ok(&format!("  {SHARE}  \n")).embed_url, SHARE);
    }

    #[test]
    fn query_and_fragment_are_dropped() {
        assert_eq!(ok(&format!("{SHARE}?collapsed=1#top")).embed_url, SHARE);
    }

    #[test]
    fn host_case_is_normalized() {
        let url = "https://WWW.LinkedIn.COM/embed/feed/update/urn:li:share:1";
        assert_eq!(
            ok(url).embed_url,
            "https://www.linkedin.com/embed/feed/update/urn:li:share:1"
        );
    }

    // ---- rejected ----

    #[test]
    fn empty_input() {
        assert_eq!(err(""), EmbedError::NoUrl);
        assert_eq!(err("   \n"), EmbedError::NoUrl);
    }

    #[test]
    fn snippet_without_src() {
        assert_eq!(err("<iframe height='600'></iframe>"), EmbedError::NoUrl);
    }

    #[test]
    fn garbage() {
        assert_eq!(err("hello"), EmbedError::InvalidUrl);
    }

    #[test]
    fn too_long() {
        assert_eq!(
            err(&format!("{SHARE}{}", "1".repeat(5000))),
            EmbedError::InvalidUrl
        );
    }

    #[test]
    fn http_and_javascript() {
        assert_eq!(err(&SHARE.replace("https", "http")), EmbedError::NotHttps);
        assert_eq!(err("javascript:alert(1)"), EmbedError::NotHttps);
    }

    #[test]
    fn wrong_hosts() {
        for host in [
            "linkedin.com",
            "evil.com",
            "www.linkedin.com.evil.com",
            "www.linkedin.com.",
            "www.linkedin.com@evil.com",
            "user@www.linkedin.com",
            "www.linkedin.com:8443",
        ] {
            let url = format!("https://{host}/embed/feed/update/urn:li:share:1");
            assert_eq!(err(&url), EmbedError::WrongHost, "{url}");
        }
    }

    #[test]
    fn wrong_paths() {
        for path in [
            "/embed/feed/update/urn:li:share:1/extra",
            "/embed/feed/update/urn:li:share:1/",
            "/embed/feed/update/urn:li:share:abc",
            "/embed/feed/update/urn:li:share:",
            "/embed/feed/update/urn:li:comment:1",
            "/embed/feed/update/urn:li:UGCPOST:1",
            "/embed/feed/update/urn:li:share:١٢٣",
            "/feed/update/urn:li:share:1",
            "/prefix/embed/feed/update/urn:li:share:1",
        ] {
            let url = format!("https://www.linkedin.com{path}");
            assert_eq!(err(&url), EmbedError::WrongPath, "{url}");
        }
    }
}
