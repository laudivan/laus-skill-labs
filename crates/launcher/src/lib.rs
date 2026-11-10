//! # `laus-launcher` — Ultralight Native Wrapper (`wry` + `tao` + `include_dir!`)
//!
//! Embeds compiled `web-shell/` assets directly into the native binary and serves them
//! over the custom `laus://localhost/` URI scheme with strict Cross-Origin Isolation headers
//! (`COOP`/`COEP`) for OPFS + WebAssembly execution.

use include_dir::{include_dir, Dir};
use serde::{Deserialize, Serialize};

/// Compile-time embedded directory containing `web-shell/` assets.
pub static EMBEDDED_WEB_SHELL: Dir<'_> = include_dir!("$OUT_DIR/embedded-web-shell");

/// Custom URI protocol scheme registered with `wry::WebViewBuilder`.
pub const CUSTOM_PROTOCOL_SCHEME: &str = "laus";

/// Resolved HTTP-like response served by the `laus://` custom protocol handler.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmbeddedAssetResponse {
    pub status_code: u16,
    pub content_type: &'static str,
    pub coop_header: &'static str,
    pub coep_header: &'static str,
    pub body: Vec<u8>,
}

/// Determines the deterministic MIME `Content-Type` header for an embedded asset path.
pub fn mime_type_for_path(path: &str) -> &'static str {
    let clean = path.split('?').next().unwrap_or(path);
    if clean.ends_with(".html") || clean.is_empty() || clean == "/" {
        "text/html; charset=utf-8"
    } else if clean.ends_with(".wasm") {
        "application/wasm"
    } else if clean.ends_with(".js") || clean.ends_with(".mjs") {
        "text/javascript; charset=utf-8"
    } else if clean.ends_with(".json") || clean.ends_with(".webmanifest") {
        "application/json; charset=utf-8"
    } else if clean.ends_with(".css") {
        "text/css; charset=utf-8"
    } else if clean.ends_with(".svg") {
        "image/svg+xml"
    } else {
        "application/octet-stream"
    }
}

/// Resolves a URI path against [`EMBEDDED_WEB_SHELL`], falling back to `index.html` for SPA navigation.
pub fn resolve_embedded_asset(uri_path: &str) -> EmbeddedAssetResponse {
    let normalized = uri_path.trim_start_matches('/');
    let lookup_path = if normalized.is_empty() {
        "index.html"
    } else {
        normalized
    };

    if let Some(file) = EMBEDDED_WEB_SHELL.get_file(lookup_path) {
        return EmbeddedAssetResponse {
            status_code: 200,
            content_type: mime_type_for_path(lookup_path),
            coop_header: "same-origin",
            coep_header: "require-corp",
            body: file.contents().to_vec(),
        };
    }

    if let Some(index_file) = EMBEDDED_WEB_SHELL.get_file("index.html") {
        return EmbeddedAssetResponse {
            status_code: 200,
            content_type: "text/html; charset=utf-8",
            coop_header: "same-origin",
            coep_header: "require-corp",
            body: index_file.contents().to_vec(),
        };
    }

    EmbeddedAssetResponse {
        status_code: 404,
        content_type: "text/plain; charset=utf-8",
        coop_header: "same-origin",
        coep_header: "require-corp",
        body: b"404 Not Found in ~ L@u Embedded Web Shell".to_vec(),
    }
}

/// Lists all filenames currently embedded in [`EMBEDDED_WEB_SHELL`].
pub fn list_embedded_files() -> Vec<String> {
    let mut files: Vec<String> = EMBEDDED_WEB_SHELL
        .files()
        .filter_map(|f| f.path().to_str().map(ToOwned::to_owned))
        .collect();
    files.sort();
    files
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedded_web_shell_contains_core_pwa_assets() {
        let files = list_embedded_files();
        assert!(files.contains(&"index.html".to_string()));
        assert!(files.contains(&"manifest.json".to_string()));
        assert!(files.contains(&"sw.js".to_string()));
    }

    #[test]
    fn custom_protocol_resolves_root_and_injects_isolation_headers() {
        let res = resolve_embedded_asset("/");
        assert_eq!(res.status_code, 200);
        assert_eq!(res.content_type, "text/html; charset=utf-8");
        assert_eq!(res.coop_header, "same-origin");
        assert_eq!(res.coep_header, "require-corp");
        assert!(!res.body.is_empty());
    }

    #[test]
    fn mime_mapping_supports_streaming_wasm() {
        assert_eq!(
            mime_type_for_path("/pkg/ui_desktop_bg.wasm"),
            "application/wasm"
        );
        assert_eq!(
            mime_type_for_path("/sw.js"),
            "text/javascript; charset=utf-8"
        );
        assert_eq!(
            mime_type_for_path("/manifest.json"),
            "application/json; charset=utf-8"
        );
    }
}
