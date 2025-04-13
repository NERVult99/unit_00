
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use tokio::sync::RwLock;
use trust_dns_server::authority::Catalog;
use trust_dns_server::store::in_memory::InMemoryAuthority;
use trust_dns_server::ServerFuture;
use trust_dns_proto::op::{MessageType, OpCode, ResponseCode};
use trust_dns_proto::rr::{LowerName, Name, Record, RecordType};
use trust_dns_proto::rr::rdata::A;
use trust_dns_client::op::Message;
use trust_dns_client::udp::UdpClientConnection;
use trust_dns_client::client::Client;
use trust_dns_proto::serialize::binary::BinDecodable;
use tokio::net::UdpSocket;
use chrono::{DateTime, Utc};

use crate::blocklist::BlockList;

pub struct DnsStats {
    pub total_queries: usize,
    pub blocked_queries: usize,
}

#[derive(Clone, serde::Serialize)]
pub struct DnsQuery {
    pub domain: String,
    pub timestamp: DateTime<Utc>,
    pub blocked: bool,
}

pub struct DnsServer {
    is_running: bool,
    blocklist: BlockList,
    stats: DnsStats,
    recent_queries: Vec<DnsQuery>,
    server_task: Option<tokio::task::JoinHandle<()>>,
}

impl DnsServer {
    pub fn new() -> Self {
        DnsServer {
            is_running: false,
            blocklist: BlockList::new(),
            stats: DnsStats {
                total_queries: 0,
                blocked_queries: 0,
            },
            recent_queries: Vec::new(),
            server_task: None,
        }
    }

    pub async fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.is_running {
            return Ok(());
        }

        let socket = UdpSocket::bind("127.0.0.1:5353").await?;
        let upstream_dns = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)), 53);
        
        let blocklist_clone = self.blocklist.clone();
        let stats_ref = Arc::new(RwLock::new(&mut self.stats));
        let recent_queries_ref = Arc::new(RwLock::new(&mut self.recent_queries));
        
        let task = tokio::spawn(async move {
            let mut buf = [0; 512];
            
            loop {
                match socket.recv_from(&mut buf).await {
                    Ok((size, src)) => {
                        let request_data = &buf[..size];
                        if let Ok(request) = Message::from_bytes(request_data) {
                            let query = request.queries().get(0).cloned();
                            
                            if let Some(query) = query {
                                let domain = query.name().to_utf8();
                                
                                // Check if domain is blocked
                                let is_blocked = blocklist_clone.is_blocked(&domain);
                                
                                let mut response = Message::new();
                                response.set_id(request.id());
                                response.set_message_type(MessageType::Response);
                                response.set_op_code(OpCode::Query);
                                response.set_authoritative(false);
                                response.add_query(query.clone());
                                
                                if is_blocked {
                                    // Send "blocked" response (empty with NXDOMAIN)
                                    response.set_response_code(ResponseCode::NXDomain);
                                    
                                    let response_bytes = response.to_vec().unwrap_or_default();
                                    let _ = socket.send_to(&response_bytes, src).await;
                                    
                                    // Update stats
                                    let stats = stats_ref.write().await;
                                    (**stats).total_queries += 1;
                                    (**stats).blocked_queries += 1;
                                    
                                    // Update recent queries
                                    let recent_queries = recent_queries_ref.write().await;
                                    if (**recent_queries).len() >= 100 {
                                        (**recent_queries).remove(0);
                                    }
                                    (**recent_queries).push(DnsQuery {
                                        domain: domain.clone(),
                                        timestamp: Utc::now(),
                                        blocked: true,
                                    });
                                } else {
                                    // Forward to upstream DNS
                                    let conn = UdpClientConnection::new(upstream_dns).unwrap();
                                    let client = Client::new(conn);
                                    
                                    // Forward the request to upstream DNS server
                                    match tokio::task::spawn_blocking(move || {
                                        client.send(request)
                                    }).await {
                                        Ok(Ok(upstream_response)) => {
                                            let response_bytes = upstream_response.to_vec().unwrap_or_default();
                                            let _ = socket.send_to(&response_bytes, src).await;
                                        }
                                        _ => {
                                            // Error handling - send a ServFail
                                            response.set_response_code(ResponseCode::ServFail);
                                            let response_bytes = response.to_vec().unwrap_or_default();
                                            let _ = socket.send_to(&response_bytes, src).await;
                                        }
                                    }
                                    
                                    // Update stats
                                    let stats = stats_ref.write().await;
                                    (**stats).total_queries += 1;
                                    
                                    // Update recent queries
                                    let recent_queries = recent_queries_ref.write().await;
                                    if (**recent_queries).len() >= 100 {
                                        (**recent_queries).remove(0);
                                    }
                                    (**recent_queries).push(DnsQuery {
                                        domain: domain.clone(),
                                        timestamp: Utc::now(),
                                        blocked: false,
                                    });
                                }
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error receiving data: {}", e);
                        break;
                    }
                }
            }
        });

        self.server_task = Some(task);
        self.is_running = true;
        Ok(())
    }

    pub async fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.is_running {
            return Ok(());
        }
        
        if let Some(task) = self.server_task.take() {
            task.abort();
        }
        
        self.is_running = false;
        Ok(())
    }

    pub fn is_running(&self) -> bool {
        self.is_running
    }
    
    pub fn get_stats(&self) -> &DnsStats {
        &self.stats
    }
    
    pub fn get_blocklist(&self) -> &BlockList {
        &self.blocklist
    }
    
    pub fn get_blocklist_mut(&mut self) -> &mut BlockList {
        &mut self.blocklist
    }
    
    pub fn get_recent_queries(&self) -> &Vec<DnsQuery> {
        &self.recent_queries
    }
}
