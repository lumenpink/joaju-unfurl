# joaju-unfurl

[![crates.io](https://img.shields.io/crates/v/joaju-unfurl.svg)](https://crates.io/crates/joaju-unfurl)
[![docs.rs](https://docs.rs/joaju-unfurl/badge.svg)](https://docs.rs/joaju-unfurl)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)
[![GOST](https://img.shields.io/badge/GOST-1.4.0-emerald.svg)](https://codeberg.org/lumenpink/gost)

A fast, secure Rust library for OpenGraph metadata extraction with SSRF defense and visual card rendering for Typst and the Web.

---

## 📖 Overview

`joaju-unfurl` provides automated, secure URL metadata unfurling:
- **SSRF Defense:** Automatically blocks connections to private and loopback addresses.
- **OpenGraph & Twitter Cards:** Resolves title, author, description, and preview images.
- **Typst Cards:** Generates ready-to-use Typst card figures for document authoring.

Detailed specifications and tutorials are located in `site/` or at [joaju-unfurl documentation](https://joaju.2lp.in).

---

## 🚀 Quick Start

Add `joaju-unfurl` to your `Cargo.toml`:

```toml
[dependencies]
joaju-unfurl = "0.2.0"
```

```rust
use joaju_unfurl::extract_metadata;

let html = "<meta property=\"og:title\" content=\"Analysis\" />";
let meta = extract_metadata(html, "https://example.org")?;
assert_eq!(meta.title.as_deref(), Some("Analysis"));
```

---

## 📋 GOST Compliance & License

Adheres to the **[GOST](https://codeberg.org/lumenpink/gost)** specification:
- **GOST Version:** `1.4.0` | **Languages:** `rust,shell` | **License:** `Apache-2.0`
