//! # `rust-pwa-builder` — Model Context Protocol (MCP) Server (`~ L@u’s Skill Labs`)
//!
//! Exposes MCP tools over JSON-RPC 2.0 for:
//! 1. `build_wasm_target`: Building and verifying `wasm32-unknown-unknown` crates.
//! 2. `audit_offline_readiness`: Auditing `web-shell/` PWA offline readiness invariants.

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fs;
use std::path::Path;
use std::process::Command;

/// JSON-RPC 2.0 request envelope used by MCP clients.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    #[serde(default)]
    pub id: Value,
    pub method: String,
    #[serde(default)]
    pub params: Value,
}

/// JSON-RPC 2.0 response envelope returned to MCP clients.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub id: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<Value>,
}

/// Individual check item in an offline readiness audit report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuditCheck {
    pub check_id: String,
    pub passed: bool,
    pub details: String,
}

/// Structured audit report produced by `audit_offline_readiness`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OfflineReadinessReport {
    pub workspace_root: String,
    pub passed: bool,
    pub score_percent: u8,
    pub checks: Vec<AuditCheck>,
}

/// Audits a workspace directory for Local-First PWA offline readiness invariants.
pub fn audit_offline_readiness(workspace_root: &Path) -> OfflineReadinessReport {
    let web_shell = workspace_root.join("web-shell");
    let mut checks = Vec::new();

    // 1. Check manifest.json standalone mode
    let manifest_path = web_shell.join("manifest.json");
    let manifest_ok = fs::read_to_string(&manifest_path)
        .map(|c| c.contains("\"display\"") && c.contains("\"standalone\""))
        .unwrap_or(false);
    checks.push(AuditCheck {
        check_id: "pwa.manifest.standalone".to_string(),
        passed: manifest_ok,
        details: if manifest_ok {
            "web-shell/manifest.json present and configured for display: standalone".to_string()
        } else {
            "web-shell/manifest.json missing or lacks display: standalone".to_string()
        },
    });

    // 2. Check sw.js Cache-First implementation
    let sw_path = web_shell.join("sw.js");
    let sw_ok = fs::read_to_string(&sw_path)
        .map(|c| c.contains("caches.match") && c.contains("caches.open"))
        .unwrap_or(false);
    checks.push(AuditCheck {
        check_id: "pwa.service_worker.cache_first".to_string(),
        passed: sw_ok,
        details: if sw_ok {
            "web-shell/sw.js implements Cache-First strategy via Cache Storage API".to_string()
        } else {
            "web-shell/sw.js missing or lacks Cache-First fetch handler".to_string()
        },
    });

    // 3. Check index.html device detection & SW registration
    let index_path = web_shell.join("index.html");
    let index_ok = fs::read_to_string(&index_path)
        .map(|c| {
            c.contains("serviceWorker")
                && c.contains("laus-ui-mobile")
                && c.contains("laus-ui-desktop")
        })
        .unwrap_or(false);
    checks.push(AuditCheck {
        check_id: "pwa.shell.adaptive_wasm_loader".to_string(),
        passed: index_ok,
        details: if index_ok {
            "web-shell/index.html switches Wasm bundles by device form-factor and registers sw.js"
                .to_string()
        } else {
            "web-shell/index.html missing adaptive Wasm switcher or serviceWorker registration"
                .to_string()
        },
    });

    // 4. Check Trunk.toml COOP/COEP headers for OPFS + SQLite VFS
    let trunk_path = web_shell.join("Trunk.toml");
    let trunk_ok = fs::read_to_string(&trunk_path)
        .map(|c| {
            c.contains("Cross-Origin-Opener-Policy") && c.contains("Cross-Origin-Embedder-Policy")
        })
        .unwrap_or(false);
    checks.push(AuditCheck {
        check_id: "pwa.trunk.coop_coep_headers".to_string(),
        passed: trunk_ok,
        details: if trunk_ok {
            "web-shell/Trunk.toml configures COOP/COEP headers for OPFS Wasm isolation".to_string()
        } else {
            "web-shell/Trunk.toml missing COOP/COEP isolation headers".to_string()
        },
    });

    let passed_count = checks.iter().filter(|c| c.passed).count();
    let total = checks.len().max(1);
    let score_percent = ((passed_count * 100) / total) as u8;

    OfflineReadinessReport {
        workspace_root: workspace_root.display().to_string(),
        passed: passed_count == checks.len(),
        score_percent,
        checks,
    }
}

/// Runs or simulates a Cargo `wasm32-unknown-unknown` target build check.
pub fn build_wasm_target(workspace_root: &Path, crate_name: &str, dry_run: bool) -> Value {
    let allowed = ["laus-core", "laus-ui-desktop", "laus-ui-mobile", "all"];
    if !allowed.contains(&crate_name) {
        return json!({
            "success": false,
            "error": format!("Unsupported crate '{}'. Expected one of: {:?}", crate_name, allowed),
        });
    }

    let mut args = vec![
        "check".to_string(),
        "--target".to_string(),
        "wasm32-unknown-unknown".to_string(),
    ];
    if crate_name == "all" {
        for pkg in ["laus-core", "laus-ui-desktop", "laus-ui-mobile"] {
            args.push("-p".to_string());
            args.push(pkg.to_string());
        }
    } else {
        args.push("-p".to_string());
        args.push(crate_name.to_string());
    }

    if dry_run {
        return json!({
            "success": true,
            "dry_run": true,
            "target": "wasm32-unknown-unknown",
            "crate": crate_name,
            "command": format!("cargo {}", args.join(" ")),
            "workspace_root": workspace_root.display().to_string(),
        });
    }

    match Command::new("cargo")
        .args(&args)
        .current_dir(workspace_root)
        .output()
    {
        Ok(output) => json!({
            "success": output.status.success(),
            "dry_run": false,
            "target": "wasm32-unknown-unknown",
            "crate": crate_name,
            "exit_code": output.status.code(),
            "stderr_preview": String::from_utf8_lossy(&output.stderr).lines().take(15).collect::<Vec<_>>(),
        }),
        Err(err) => json!({
            "success": false,
            "error": err.to_string(),
        }),
    }
}

/// Handles a single JSON-RPC 2.0 MCP request and returns the corresponding response.
pub fn handle_mcp_request(req: &JsonRpcRequest, default_workspace: &Path) -> JsonRpcResponse {
    match req.method.as_str() {
        "initialize" => JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id: req.id.clone(),
            result: Some(json!({
                "protocolVersion": "2024-11-05",
                "serverInfo": {
                    "name": "rust-pwa-builder",
                    "version": "0.1.0",
                    "author": "Laudivan Freire de Almeida (~ L@u / Uncle Lau)"
                },
                "capabilities": {
                    "tools": {}
                }
            })),
            error: None,
        },
        "tools/list" => JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id: req.id.clone(),
            result: Some(json!({
                "tools": [
                    {
                        "name": "build_wasm_target",
                        "description": "Compiles and verifies Rust crates (laus-core, laus-ui-desktop, laus-ui-mobile) for the wasm32-unknown-unknown WebAssembly target.",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "crate_name": {
                                    "type": "string",
                                    "description": "Crate name: 'laus-core', 'laus-ui-desktop', 'laus-ui-mobile', or 'all'"
                                },
                                "dry_run": {
                                    "type": "boolean",
                                    "description": "If true, validates arguments and returns the exact cargo command without spawning a compiler process"
                                }
                            },
                            "required": ["crate_name"]
                        }
                    },
                    {
                        "name": "audit_offline_readiness",
                        "description": "Audits web-shell/ (index.html, manifest.json, sw.js, Trunk.toml) for Cache-First Service Worker, standalone PWA manifest, adaptive Wasm bundle switching, and COOP/COEP headers.",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "workspace_path": {
                                    "type": "string",
                                    "description": "Optional path to the repository root"
                                }
                            }
                        }
                    }
                ]
            })),
            error: None,
        },
        "tools/call" => {
            let tool_name = req
                .params
                .get("name")
                .and_then(Value::as_str)
                .unwrap_or_default();
            let args = req.params.get("arguments").cloned().unwrap_or(json!({}));

            match tool_name {
                "audit_offline_readiness" => {
                    let root = args
                        .get("workspace_path")
                        .and_then(Value::as_str)
                        .map(Path::new)
                        .unwrap_or(default_workspace);
                    let report = audit_offline_readiness(root);
                    JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id: req.id.clone(),
                        result: Some(json!({
                            "content": [{
                                "type": "text",
                                "text": serde_json::to_string_pretty(&report).unwrap_or_default()
                            }],
                            "report": report
                        })),
                        error: None,
                    }
                }
                "build_wasm_target" => {
                    let crate_name = args
                        .get("crate_name")
                        .and_then(Value::as_str)
                        .unwrap_or("all");
                    let dry_run = args.get("dry_run").and_then(Value::as_bool).unwrap_or(true);
                    let outcome = build_wasm_target(default_workspace, crate_name, dry_run);
                    JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        id: req.id.clone(),
                        result: Some(json!({
                            "content": [{
                                "type": "text",
                                "text": serde_json::to_string_pretty(&outcome).unwrap_or_default()
                            }],
                            "outcome": outcome
                        })),
                        error: None,
                    }
                }
                unknown => JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: req.id.clone(),
                    result: None,
                    error: Some(json!({
                        "code": -32601,
                        "message": format!("Unknown MCP tool: {}", unknown)
                    })),
                },
            }
        }
        other => JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id: req.id.clone(),
            result: None,
            error: Some(json!({
                "code": -32601,
                "message": format!("Method not found: {}", other)
            })),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn repo_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .to_path_buf()
    }

    #[test]
    fn offline_readiness_audit_passes_on_repository_web_shell() {
        let root = repo_root();
        let report = audit_offline_readiness(&root);
        assert!(
            report.passed,
            "Expected all offline readiness checks to pass: {:?}",
            report.checks
        );
        assert_eq!(report.score_percent, 100);
    }

    #[test]
    fn mcp_jsonrpc_tools_list_and_call_work() {
        let root = repo_root();
        let list_req = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: json!(1),
            method: "tools/list".to_string(),
            params: json!({}),
        };
        let list_res = handle_mcp_request(&list_req, &root);
        assert!(list_res.error.is_none());

        let call_req = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: json!(2),
            method: "tools/call".to_string(),
            params: json!({
                "name": "build_wasm_target",
                "arguments": {
                    "crate_name": "laus-core",
                    "dry_run": true
                }
            }),
        };
        let call_res = handle_mcp_request(&call_req, &root);
        assert!(call_res.error.is_none());
        let result = call_res.result.unwrap();
        assert_eq!(result["outcome"]["success"], json!(true));
    }
}
