
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use tauri::State;

use crate::dns_server::DnsServer;

#[tauri::command]
pub async fn start_dns_server(dns_server: State<'_, Arc<Mutex<DnsServer>>>) -> Result<bool, String> {
    let mut server = dns_server.lock().await;
    server.start().await.map_err(|e| e.to_string())?;
    Ok(true)
}

#[tauri::command]
pub async fn stop_dns_server(dns_server: State<'_, Arc<Mutex<DnsServer>>>) -> Result<bool, String> {
    let mut server = dns_server.lock().await;
    server.stop().await.map_err(|e| e.to_string())?;
    Ok(true)
}

#[tauri::command]
pub async fn get_dns_status(dns_server: State<'_, Arc<Mutex<DnsServer>>>) -> Result<bool, String> {
    let server = dns_server.lock().await;
    Ok(server.is_running())
}

#[tauri::command]
pub async fn add_to_blocklist(domain: String, dns_server: State<'_, Arc<Mutex<DnsServer>>>) -> Result<(), String> {
    let mut server = dns_server.lock().await;
    server.get_blocklist_mut().add(domain);
    Ok(())
}

#[tauri::command]
pub async fn remove_from_blocklist(domain: String, dns_server: State<'_, Arc<Mutex<DnsServer>>>) -> Result<(), String> {
    let mut server = dns_server.lock().await;
    server.get_blocklist_mut().remove(&domain);
    Ok(())
}

#[derive(Serialize)]
pub struct StatsResponse {
    total_queries: usize,
    blocked_queries: usize,
    blocklist_size: usize,
}

#[tauri::command]
pub async fn get_stats(dns_server: State<'_, Arc<Mutex<DnsServer>>>) -> Result<StatsResponse, String> {
    let server = dns_server.lock().await;
    let stats = server.get_stats();
    let blocklist_size = server.get_blocklist().count();
    
    Ok(StatsResponse {
        total_queries: stats.total_queries,
        blocked_queries: stats.blocked_queries,
        blocklist_size,
    })
}

#[tauri::command]
pub async fn load_blocklist_from_file(
    file_path: String, 
    dns_server: State<'_, Arc<Mutex<DnsServer>>>
) -> Result<usize, String> {
    let mut server = dns_server.lock().await;
    server.get_blocklist_mut()
        .load_from_file(file_path)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_recent_queries(dns_server: State<'_, Arc<Mutex<DnsServer>>>) -> Result<Vec<crate::dns_server::DnsQuery>, String> {
    let server = dns_server.lock().await;
    Ok(server.get_recent_queries().clone())
}
