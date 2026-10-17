use unfurl_rs::{extract_metadata, is_ssrf_risk, render_unfurl_typst_card, ExtractedMetadata};

#[test]
fn test_ssrf_blocking() {
    assert!(is_ssrf_risk("http://localhost:8080"));
    assert!(is_ssrf_risk("http://127.0.0.1/admin"));
    assert!(is_ssrf_risk("http://[::1]/secret"));
    assert!(is_ssrf_risk("http://10.0.0.1/"));
    assert!(is_ssrf_risk("http://192.168.1.1/router"));
    assert!(is_ssrf_risk("http://172.16.0.1/"));
    assert!(is_ssrf_risk("http://meu-host.local/"));
    assert!(is_ssrf_risk("file:///etc/passwd"));
    assert!(is_ssrf_risk("javascript:alert(1)"));

    // URLs públicas legítimas não devem ser bloqueadas
    assert!(!is_ssrf_risk("https://codeberg.org/lumenpink/joaju"));
    assert!(!is_ssrf_risk("https://rust-lang.org"));
}

#[test]
fn test_metadata_extraction_opengraph_and_twitter() {
    let html = r#"<!DOCTYPE html>
<html>
<head>
    <title>Fallback Title</title>
    <meta property="og:title" content="Artigo de Genética Médica">
    <meta property="og:description" content="Estudo sobre herança autossômica em famílias brasileiras.">
    <meta property="og:site_name" content="Revista Brasileira de Genética">
    <meta property="og:image" content="https://exemplo.org/imagem.png">
    <meta name="author" content="Dra. Lumen Pink">
    <link rel="icon" href="/favicon.ico">
</head>
<body>
    <article>
        <p>Conteúdo relevante do artigo com várias palavras para cálculo de tempo de leitura aproximado.</p>
    </article>
</body>
</html>"#;

    let meta =
        extract_metadata(html, "https://exemplo.org/artigo").expect("falha ao extrair metadados");
    assert_eq!(meta.title.as_deref(), Some("Artigo de Genética Médica"));
    assert_eq!(
        meta.description.as_deref(),
        Some("Estudo sobre herança autossômica em famílias brasileiras.")
    );
    assert_eq!(
        meta.site_name.as_deref(),
        Some("Revista Brasileira de Genética")
    );
    assert_eq!(meta.author.as_deref(), Some("Dra. Lumen Pink"));
    assert_eq!(
        meta.image_url.as_deref(),
        Some("https://exemplo.org/imagem.png")
    );
    assert_eq!(
        meta.favicon_url.as_deref(),
        Some("https://exemplo.org/favicon.ico")
    );
    assert_eq!(meta.domain, "exemplo.org");
}

#[test]
fn test_render_typst_card() {
    let meta = ExtractedMetadata {
        domain: "codeberg.org".to_string(),
        site_name: Some("Codeberg".to_string()),
        title: Some("Joaju Compiler".to_string()),
        description: Some("Compilador acadêmico declarativo".to_string()),
        image_url: None,
        favicon_url: None,
        author: Some("Lumen Pink".to_string()),
        date: None,
        reading_time_min: Some(3),
        has_article: true,
    };

    let card = render_unfurl_typst_card(&meta, "https://codeberg.org/lumenpink/joaju");
    assert!(card.contains("#rect("));
    assert!(card.contains("Joaju Compiler"));
    assert!(card.contains("codeberg.org"));
    assert!(card.contains("Compilador acadêmico declarativo"));
}
