//! # joaju-unfurl
//!
//! Motor soberano em Rust para extração de metadados OpenGraph com blindagem contra
//! Server-Side Request Forgery (SSRF) e renderizador de cartões visuais para Typst.
//!
//! > ⚠️ **Early Development Notice (pre-1.0 / alpha)**: This library is currently in early
//! > active development (`0.1.0`). It is NOT recommended for production use beyond experimental
//! > testing and research. APIs are subject to breaking changes prior to a stable 1.0.0 release.
//!
//! ## Exemplo Rápido
//!
//! ```rust
//! use joaju_unfurl::extract_metadata;
//!
//! let html = r#"<title>Exemplo</title><meta property="og:description" content="Desc">"#;
//! let meta = extract_metadata(html, "https://exemplo.org").unwrap();
//! assert_eq!(meta.title.as_deref(), Some("Exemplo"));
//! assert_eq!(meta.domain, "exemplo.org");
//! ```

use reqwest::Url;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UnfurlError {
    #[error("URL inválida ou ausente: {0}")]
    InvalidUrl(String),

    #[error("Acesso bloqueado por segurança (SSRF): {0}")]
    SsrfBlocked(String),

    #[error("Erro de requisição HTTP: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Erro de parsing")]
    ParseError,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtractedMetadata {
    pub domain: String,
    pub site_name: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub favicon_url: Option<String>,
    pub author: Option<String>,
    pub date: Option<String>,
    pub reading_time_min: Option<u32>,
    pub has_article: bool,
}

#[derive(Debug, Deserialize)]
pub struct UnfurlRequest {
    pub url: Option<String>,
    #[serde(default)]
    pub archive: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnfurlResponse {
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "siteName", skip_serializing_if = "Option::is_none")]
    pub site_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "imageUrl", skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(rename = "faviconUrl", skip_serializing_if = "Option::is_none")]
    pub favicon_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(rename = "readingTimeMin", skip_serializing_if = "Option::is_none")]
    pub reading_time_min: Option<u32>,
    #[serde(rename = "hasArticle", skip_serializing_if = "Option::is_none")]
    pub has_article: Option<bool>,
    #[serde(rename = "pdfBase64", skip_serializing_if = "Option::is_none")]
    pub pdf_base64: Option<String>,
}

pub fn is_ssrf_risk(url_str: &str) -> bool {
    let parsed = match Url::parse(url_str) {
        Ok(u) => u,
        Err(_) => return true,
    };

    let scheme = parsed.scheme();
    if scheme != "http" && scheme != "https" {
        return true;
    }

    if let Some(host) = parsed.host_str() {
        let host_lower = host.to_lowercase();
        if host_lower == "localhost"
            || host_lower == "127.0.0.1"
            || host_lower == "::1"
            || host_lower == "[::1]"
            || host_lower == "0.0.0.0"
            || host_lower.starts_with("192.168.")
            || host_lower.starts_with("10.")
            || host_lower.ends_with(".local")
        {
            return true;
        }

        if host_lower.starts_with("172.") {
            let parts: Vec<&str> = host_lower.split('.').collect();
            if parts.len() >= 2 {
                if let Ok(second_octet) = parts[1].parse::<u8>() {
                    if (16..=31).contains(&second_octet) {
                        return true;
                    }
                }
            }
        }
    } else {
        return true;
    }

    false
}

pub fn extract_metadata(html: &str, base_url: &str) -> Result<ExtractedMetadata, UnfurlError> {
    let parsed_url =
        Url::parse(base_url).map_err(|_| UnfurlError::InvalidUrl(base_url.to_string()))?;
    let domain = parsed_url.host_str().unwrap_or("").to_string();

    let document = Html::parse_document(html);

    let get_meta = |property: &str, name: &str| -> Option<String> {
        let sel_prop = Selector::parse(&format!("meta[property='{}']", property)).ok()?;
        if let Some(el) = document.select(&sel_prop).next() {
            if let Some(content) = el.value().attr("content") {
                if !content.trim().is_empty() {
                    return Some(content.trim().to_string());
                }
            }
        }

        let sel_name = Selector::parse(&format!("meta[name='{}']", name)).ok()?;
        if let Some(el) = document.select(&sel_name).next() {
            if let Some(content) = el.value().attr("content") {
                if !content.trim().is_empty() {
                    return Some(content.trim().to_string());
                }
            }
        }

        None
    };

    let title = get_meta("og:title", "twitter:title").or_else(|| {
        let sel = Selector::parse("title").ok()?;
        document
            .select(&sel)
            .next()
            .map(|el| el.text().collect::<String>().trim().to_string())
            .filter(|s| !s.is_empty())
    });

    let description = get_meta("og:description", "description");
    let site_name = get_meta("og:site_name", "application-name");
    let author =
        get_meta("article:author", "author").or_else(|| get_meta("og:article:author", "author"));
    let date = get_meta("article:published_time", "date");

    let image_url = get_meta("og:image", "twitter:image")
        .and_then(|img| parsed_url.join(&img).ok().map(|u| u.to_string()));

    let favicon_url = Selector::parse("link[rel*='icon']").ok().and_then(|sel| {
        document
            .select(&sel)
            .next()
            .and_then(|el| el.value().attr("href"))
            .and_then(|href| parsed_url.join(href).ok().map(|u| u.to_string()))
    });

    let sel_p = Selector::parse("p").ok();
    let text_content = sel_p
        .map(|sel| {
            document
                .select(&sel)
                .map(|el| el.text().collect::<String>())
                .collect::<Vec<_>>()
                .join(" ")
        })
        .unwrap_or_default();

    let word_count = text_content.split_whitespace().count();
    let reading_time_min = if word_count > 0 {
        Some(((word_count as f64 / 200.0).ceil() as u32).max(1))
    } else {
        None
    };

    let has_article = Selector::parse("article")
        .ok()
        .map(|sel| document.select(&sel).next().is_some())
        .unwrap_or(false);

    Ok(ExtractedMetadata {
        domain,
        site_name,
        title,
        description,
        image_url,
        favicon_url,
        author,
        date,
        reading_time_min,
        has_article,
    })
}

pub async fn fetch_and_unfurl(url_str: &str) -> Result<ExtractedMetadata, UnfurlError> {
    if is_ssrf_risk(url_str) {
        return Err(UnfurlError::SsrfBlocked(url_str.to_string()));
    }

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .build()?;

    let resp = client.get(url_str).send().await?;
    let html = resp.text().await?;

    extract_metadata(&html, url_str)
}

pub fn render_unfurl_typst_card(meta: &ExtractedMetadata, original_url: &str) -> String {
    let title = meta.title.as_deref().unwrap_or("Sem título");
    let desc = meta.description.as_deref().unwrap_or("");
    let domain = &meta.domain;

    format!(
        "#rect(width: 100%, stroke: 0.5pt + luma(180), radius: 6pt, inset: 10pt)[\n  #link(\"{}\")[\n    #text(weight: \"bold\", size: 10.5pt)[{}]\n    #v(-4pt)\n    #text(fill: luma(100), size: 8.5pt)[{}]\n    #v(-2pt)\n    #text(size: 9pt)[{}]\n  ]\n]",
        original_url, title, domain, desc
    )
}
