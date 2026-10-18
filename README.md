# joaju-unfurl

[![crates.io](https://img.shields.io/crates/v/joaju-unfurl.svg)](https://crates.io/crates/joaju-unfurl)
[![docs.rs](https://docs.rs/joaju-unfurl/badge.svg)](https://docs.rs/joaju-unfurl)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)
[![GOST](https://img.shields.io/badge/GOST-1.1.0-emerald.svg)](https://codeberg.org/lumenpink/gost)

Motor soberano em Rust para extração determinística de metadados OpenGraph com blindagem contra Server-Side Request Forgery (SSRF) e renderizador de cartões visuais para Typst e Web.

> ⚠️ **Early Development Notice (pre-1.0 / alpha)**  
> This library is currently in early active development (`0.1.0`). It is NOT recommended for production use beyond experimental testing and research. APIs are subject to breaking changes prior to a stable 1.0.0 release.

---

## 📖 Visão Geral

`joaju-unfurl` oferece uma solução robusta e segura para pré-visualização de links (unfurling), muito comum em aplicativos de mensagens, fóruns e geradores de documentos (como Typst e LaTeX).

### Principais Funcionalidades:
- **Blindagem Anti-SSRF:** Bloqueia automaticamente acessos a endereços privados (`127.0.0.1`, `localhost`, redes `10.0.0.0/8`, `172.16.0.0/12`, `192.168.0.0/16`, `::1` e domínios `.local`).
- **Resolução Semântica OpenGraph e Twitter Cards:** Extrai título, descrição, autor, data, imagem de capa, favicon e estima o tempo de leitura.
- **Parser Resiliente HTML:** Suporte a parsing a partir de HTML estático em memória ou via requisição HTTP assíncrona com timeout estrito e User-Agent controlado.
- **Cartões Visuais em Typst:** Geração direta de código Typst pronto para inclusão em documentos.

---

## 🚀 Instalação

Adicione `joaju-unfurl` ao seu `Cargo.toml`:

```toml
[dependencies]
joaju-unfurl = "0.1.0"
```

Ou instale a CLI:

```bash
cargo install joaju-unfurl
```

---

## 🛠️ Exemplo de Uso (Rust API)

### 1. Extração direta a partir de HTML em memória (Zero I/O)

```rust
use joaju_unfurl::extract_metadata;

let html = r#"
<!DOCTYPE html>
<html>
<head>
    <meta property="og:title" content="Artigo Revolucionário" />
    <meta property="og:description" content="Uma análise aprofundada sobre sistemas soberanos." />
    <meta property="og:site_name" content="Diário Tecnológico" />
    <meta name="author" content="Lumen Pink" />
</head>
<body>
    <article>
        <p>Conteúdo relevante do artigo com várias palavras...</p>
    </article>
</body>
</html>
"#;

let meta = extract_metadata(html, "https://exemplo.org/artigo").unwrap();
assert_eq!(meta.title.as_deref(), Some("Artigo Revolucionário"));
assert_eq!(meta.domain, "exemplo.org");
assert_eq!(meta.author.as_deref(), Some("Lumen Pink"));
assert!(meta.has_article);
```

### 2. Requisição Assíncrona com Blindagem SSRF

```rust,no_run
use joaju_unfurl::{fetch_and_unfurl, render_unfurl_typst_card};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://codeberg.org";
    let meta = fetch_and_unfurl(url).await?;

    println!("Título: {:?}", meta.title);
    println!("Domínio: {}", meta.domain);

    // Gerar cartão Typst
    let typst_card = render_unfurl_typst_card(&meta, url);
    println!("\nCartão Typst:\n{}", typst_card);

    Ok(())
}
```

---

## 💻 Uso via CLI

```bash
# Extrair metadados e retornar JSON
joaju-unfurl fetch "https://exemplo.org"

# Gerar cartão para documento Typst
joaju-unfurl card "https://exemplo.org"
```

---

## 📋 Conformidade GOST

Este projeto adota a norma **[GOST](https://codeberg.org/lumenpink/gost)** (Gosplan Standard for Technical Software).
- **Versão GOST:** `1.1.0`
- **Linguagens declaradas:** `rust,shell`
- **Licença:** `Apache-2.0`

```bash
# Executar auditoria de qualidade estrita
cccp check --strict
```

---

## 📄 Licença

Distribuído sob a licença [Apache-2.0](./LICENSE).
