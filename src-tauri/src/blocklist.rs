
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Clone)]
pub struct BlockList {
    domains: HashSet<String>,
}

impl BlockList {
    pub fn new() -> Self {
        BlockList {
            domains: HashSet::new(),
        }
    }
    
    pub fn add(&mut self, domain: String) {
        self.domains.insert(domain);
    }
    
    pub fn remove(&mut self, domain: &str) {
        self.domains.remove(domain);
    }
    
    pub fn is_blocked(&self, domain: &str) -> bool {
        // Check exact match
        if self.domains.contains(domain) {
            return true;
        }
        
        // Check if domain is a subdomain of a blocked domain
        let parts: Vec<&str> = domain.split('.').collect();
        for i in 0..parts.len() - 1 {
            let parent_domain = parts[i..].join(".");
            if self.domains.contains(&parent_domain) {
                return true;
            }
        }
        
        false
    }
    
    pub fn load_from_file<P: AsRef<Path>>(&mut self, path: P) -> Result<usize, std::io::Error> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        
        let mut count = 0;
        
        for line in reader.lines() {
            if let Ok(line) = line {
                let line = line.trim();
                
                // Skip comments and empty lines
                if line.is_empty() || line.starts_with('#') {
                    continue;
                }
                
                // Parse line (handles common formats like "0.0.0.0 example.com")
                let domain = line
                    .split_whitespace()
                    .nth(1)
                    .unwrap_or(line)
                    .to_lowercase();
                
                if !domain.is_empty() {
                    self.add(domain);
                    count += 1;
                }
            }
        }
        
        Ok(count)
    }
    
    pub fn get_all_domains(&self) -> Vec<String> {
        self.domains.iter().cloned().collect()
    }
    
    pub fn count(&self) -> usize {
        self.domains.len()
    }
}
