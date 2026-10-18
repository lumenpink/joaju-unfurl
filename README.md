# joaju-unfurl

[![crates.io](https://img.shields.io/crates/v/joaju-unfurl.svg)](https://crates.io/crates/joaju-unfurl)
[![docs.rs](https://docs.rs/joaju-unfurl/badge.svg)](https://docs.rs/joaju-unfurl)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)
[![GOST](https://img.shields.io/badge/GOST-1.1.0-emerald.svg)](https://codeberg.org/lumenpink/gost)

A fast and secure Rust library for deterministic OpenGraph metadata extraction with Server-Side Request Forgery (SSRF) defensive protection and visual card rendering for Typst and the Web.

> ⚠️ **Early Development Notice (pre-1.0 / alpha)**  
> This library is currently in early active development (`0.1.0`). It is NOT recommended for production use beyond experimental testing and research. APIs are subject to breaking changes prior to a stable 1.0.0 release.

---

## 📖 Overview

`joaju-unfurl` provides a robust and secure solution for URL preview generation (link unfurling), commonly utilized in messaging platforms, forums, and document generation pipelines (such as Typst and LaTeX).

### Key Features:
- **SSRF Defense:** Automatically blocks connections to private and loopback addresses (`127.0.0.1`, `localhost`, `10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16`, `::1`, and `.local` domains).
- **Semantic OpenGraph & Twitter Cards Resolution:** Extracts title, description, author, publication date, cover image, favicon, and estimates reading time.
- **Resilient HTML Parser:** Supports parsing from in-memory static HTML strings or via asynchronous HTTP requests with strict timeouts and managed User-Agent headers.
- **Typst Visual Cards:** Direct code emission ready for document compilation.

---

## 🚀 Installation

Add `joaju-unfurl` to your `Cargo.toml`:

```toml
[dependencies]
joaju-unfurl = "0.1.0"
```

Or install the CLI tool:

```bash
cargo install joaju-unfurl
```

---

## 🛠️ Usage Example (Rust API)

### 1. In-Memory Extraction from Static HTML (Zero I/O)

```rust
use joaju_unfurl::extract_metadata;

let html = r#"
<!DOCTYPE html>
<html>
<head>
    <meta property="og:title" content="Engineering Systems Overview" />
    <meta property="og:description" content="An in-depth analysis of distributed systems architectures." />
    <meta property="og:site_name" content="Tech Journal" />
    <meta name="author" content="Lumen Pink" />
</head>
<body>
    <article>
        <p>Comprehensive article content with multiple paragraphs and analysis...</p>
    </article>
</body>
</html>
"#;

let meta = extract_metadata(html, "https://example.org/article").unwrap();
assert_eq!(meta.title.as_deref(), Some("Engineering Systems Overview"));
assert_eq!(meta.domain, "example.org");
assert_eq!(meta.author.as_deref(), Some("Lumen Pink"));
assert!(meta.has_article);
```

### 2. Asynchronous HTTP Request with SSRF Protection

```rust,no_run
use joaju_unfurl::{fetch_and_unfurl, render_unfurl_typst_card};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://codeberg.org";
    let meta = fetch_and_unfurl(url).await?;

    println!("Title: {:?}", meta.title);
    println!("Domain: {}", meta.domain);

    // Generate Typst preview card
    let typst_card = render_unfurl_typst_card(&meta, url);
    println!("\nTypst Card:\n{}", typst_card);

    Ok(())
}
```

---

## 💻 CLI Usage

```bash
# Extract metadata and print JSON
joaju-unfurl --url "https://example.org"

# Generate visual card for Typst documents
joaju-unfurl --url "https://example.org" --typst
```

---

## 📋 GOST Compliance

This project adheres to the **[GOST](https://codeberg.org/lumenpink/gost)** specification (Gosplan Standard for Technical Software).
- **GOST Version:** `1.1.0`
- **Declared Languages:** `rust,shell`
- **License:** `Apache-2.0`

```bash
# Run strict compliance verification
cccp check --strict
```

---

## 📄 License

Distributed under the [Apache-2.0](./LICENSE) license.
