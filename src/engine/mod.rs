// engine/mod.rs
use std::collections::VecDeque;

pub struct BrowserEngine {
    history: VecDeque<String>,
    current_index: isize,
    current_tab: usize,
    tabs: Vec<Tab>,
}

struct Tab {
    url: String,
    title: String,
    // Add webview handle here later
}

impl BrowserEngine {
    pub fn new() -> Self {
        Self {
            history: VecDeque::new(),
            current_index: -1,
            current_tab: 0,
            tabs: vec![Tab {
                url: "about:blank".to_string(),
                title: "New Tab".to_string(),
            }],
        }
    }

    pub fn navigate_to(&mut self, url: &str) -> Result<String, Box<dyn std::error::Error>> {
        // Validate and normalize URL
        let final_url = self.normalize_url(url)?;
        
        // Add to history
        self.history.push_back(final_url.clone());
        self.current_index = self.history.len() as isize - 1;
        
        // Update current tab
        if let Some(tab) = self.tabs.get_mut(self.current_tab) {
            tab.url = final_url.clone();
            // TODO: Load actual web content here
        }
        
        Ok(final_url)
    }

    pub fn go_back(&mut self) -> Option<String> {
        if self.can_go_back() {
            self.current_index -= 1;
            Some(self.history[self.current_index as usize].clone())
        } else {
            None
        }
    }

    pub fn go_forward(&mut self) -> Option<String> {
        if self.can_go_forward() {
            self.current_index += 1;
            Some(self.history[self.current_index as usize].clone())
        } else {
            None
        }
    }

    pub fn can_go_back(&self) -> bool {
        self.current_index > 0
    }

    pub fn can_go_forward(&self) -> bool {
        self.current_index < (self.history.len() as isize - 1)
    }

    pub fn reload(&mut self) {
        // TODO: Implement reload logic
        println!("Reloading current page");
    }

    pub fn new_tab(&mut self) {
        self.tabs.push(Tab {
            url: "about:blank".to_string(),
            title: "New Tab".to_string(),
        });
        self.current_tab = self.tabs.len() - 1;
    }

    fn normalize_url(&self, url: &str) -> Result<String, Box<dyn std::error::Error>> {
        // Basic URL normalization
        if url.starts_with("http://") || url.starts_with("https://") {
            Ok(url.to_string())
        } else if url.contains('.') && !url.contains(' ') {
            Ok(format!("https://{}", url))
        } else {
            // Treat as search query
            Ok(format!("https://www.google.com/search?q={}", urlencoding::encode(url)))
        }
    }
}