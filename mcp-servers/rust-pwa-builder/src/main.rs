use rust_pwa_builder::{
    audit_offline_readiness, build_wasm_target, handle_mcp_request, JsonRpcRequest,
};
use std::env;
use std::io::{self, BufRead, Write};
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();
    let workspace_root = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));

    if args.iter().any(|a| a == "--help" || a == "-h") {
        println!("rust-pwa-builder v0.1.0 — ~ L@u’s Skill Labs MCP Server");
        println!("Usage:");
        println!("  rust-pwa-builder               Run MCP JSON-RPC 2.0 server over stdio");
        println!("  rust-pwa-builder --audit       Run offline readiness audit on workspace");
        println!("  rust-pwa-builder --check-wasm  Validate wasm32 build configuration");
        return;
    }

    if args.iter().any(|a| a == "--audit") {
        let report = audit_offline_readiness(&workspace_root);
        println!(
            "{}",
            serde_json::to_string_pretty(&report).expect("failed to serialize audit report")
        );
        std::process::exit(if report.passed { 0 } else { 1 });
    }

    if args.iter().any(|a| a == "--check-wasm") {
        let res = build_wasm_target(&workspace_root, "all", true);
        println!(
            "{}",
            serde_json::to_string_pretty(&res).expect("failed to serialize build check")
        );
        return;
    }

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    for line in stdin.lock().lines() {
        let Ok(line) = line else { break };
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        match serde_json::from_str::<JsonRpcRequest>(trimmed) {
            Ok(req) => {
                let res = handle_mcp_request(&req, &workspace_root);
                if let Ok(json_out) = serde_json::to_string(&res) {
                    let _ = writeln!(stdout, "{}", json_out);
                    let _ = stdout.flush();
                }
            }
            Err(err) => {
                let err_payload = serde_json::json!({
                    "jsonrpc": "2.0",
                    "id": null,
                    "error": {
                        "code": -32700,
                        "message": format!("Parse error: {}", err)
                    }
                });
                let _ = writeln!(stdout, "{}", err_payload);
                let _ = stdout.flush();
            }
        }
    }
}
