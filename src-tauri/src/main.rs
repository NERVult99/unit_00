
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod dns_server;
mod blocklist;
mod commands;

use dns_server::DnsServer;
use std::sync::Arc;
use tokio::sync::Mutex;

fn main() {
    env_logger::init();
    
    let dns_server = Arc::new(Mutex::new(DnsServer::new()));
    let dns_server_clone = dns_server.clone();

    // Start the DNS server in a background task
    std::thread::spawn(move || {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.block_on(async {
            let mut server = dns_server_clone.lock().await;
            // Don't start automatically at launch - let the user enable it
            // server.start().await.unwrap();
        });
    });
    
    tauri::Builder::default()
        .manage(dns_server)
        .invoke_handler(tauri::generate_handler![
            commands::start_dns_server,
            commands::stop_dns_server,
            commands::get_dns_status,
            commands::add_to_blocklist,
            commands::remove_from_blocklist,
            commands::get_stats,
            commands::load_blocklist_from_file,
            commands::get_recent_queries
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
