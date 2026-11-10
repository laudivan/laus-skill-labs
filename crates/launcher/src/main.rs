use laus_core::{OfflineStateEngine, PROJECT_IDENTIFIER};
use laus_launcher::{list_embedded_files, resolve_embedded_asset, CUSTOM_PROTOCOL_SCHEME};

fn main() {
    let mut engine = OfflineStateEngine::new("laus-native-launcher");
    engine
        .upsert_document(
            "launcher/boot",
            "Native Wry/Tao Boot Check",
            "Embedded web-shell verified via include_dir!",
        )
        .expect("boot state upsert must succeed");

    let embedded_files = list_embedded_files();
    let root_response = resolve_embedded_asset("/index.html");

    println!("============================================================");
    println!("  {}", PROJECT_IDENTIFIER);
    println!("  Ultralight Native Desktop Launcher (wry / tao / include_dir!)");
    println!("============================================================");
    println!("Protocol Scheme : {}://localhost/", CUSTOM_PROTOCOL_SCHEME);
    println!("Embedded Assets : {:?}", embedded_files);
    println!(
        "Index Payload   : {} bytes ({}, COOP={}, COEP={})",
        root_response.body.len(),
        root_response.content_type,
        root_response.coop_header,
        root_response.coep_header
    );
    println!(
        "Offline Engine  : Node '{}' @ Lamport tick {} ({} Outbox pending)",
        engine.node_id(),
        engine.current_tick(),
        engine.pending_sync_count()
    );
}
