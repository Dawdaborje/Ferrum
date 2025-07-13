use eframe::egui;

fn main() {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    let _ = eframe::run_native("Ferrum Browser", native_options, Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))));
}

#[derive(Clone)]
struct Tab {
    id: usize,
    title: String,
    url: String,
    loading: bool,
    favicon: Option<String>,
}

struct MyEguiApp {
    tabs: Vec<Tab>,
    active_tab: usize,
    next_tab_id: usize,
    address_bar_text: String,
    bookmarks: Vec<Bookmark>,
    history: Vec<String>,
    show_bookmarks: bool,
    show_history: bool,
    show_settings: bool,
}

#[derive(Clone)]
struct Bookmark {
    title: String,
    url: String,
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let _ = cc;
        
        // Create initial tab
        let initial_tab = Tab {
            id: 0,
            title: "New Tab".to_string(),
            url: "about:blank".to_string(),
            loading: false,
            favicon: None,
        };
        
        // Sample bookmarks
        let bookmarks = vec![
            Bookmark { title: "GitHub".to_string(), url: "https://github.com".to_string() },
            Bookmark { title: "Rust".to_string(), url: "https://rust-lang.org".to_string() },
            Bookmark { title: "egui".to_string(), url: "https://github.com/emilk/egui".to_string() },
        ];
        
        Self {
            tabs: vec![initial_tab],
            active_tab: 0,
            next_tab_id: 1,
            address_bar_text: "about:blank".to_string(),
            bookmarks,
            history: Vec::new(),
            show_bookmarks: false,
            show_history: false,
            show_settings: false,
        }
    }
    
    fn add_new_tab(&mut self) {
        let new_tab = Tab {
            id: self.next_tab_id,
            title: "New Tab".to_string(),
            url: "about:blank".to_string(),
            loading: false,
            favicon: None,
        };
        self.tabs.push(new_tab);
        self.active_tab = self.tabs.len() - 1;
        self.next_tab_id += 1;
        self.address_bar_text = "about:blank".to_string();
    }
    
    fn close_tab(&mut self, index: usize) {
        if self.tabs.len() > 1 {
            self.tabs.remove(index);
            if self.active_tab >= index && self.active_tab > 0 {
                self.active_tab -= 1;
            }
            if self.active_tab >= self.tabs.len() {
                self.active_tab = self.tabs.len() - 1;
            }
            self.update_address_bar();
        }
    }
    
    fn navigate_to(&mut self, url: String) {
        let title = self.get_title_from_url(&url);
        
        if let Some(tab) = self.tabs.get_mut(self.active_tab) {
            tab.url = url.clone();
            tab.loading = true;
            tab.title = "Loading...".to_string();
            self.address_bar_text = url.clone();
            
            // Add to history
            if !self.history.contains(&url) {
                self.history.push(url.clone());
            }
            
            // Simulate loading completion
            tab.loading = false;
            tab.title = title;
        }
    }
    
    fn get_title_from_url(&self, url: &str) -> String {
        if url == "about:blank" {
            "New Tab".to_string()
        } else if url.starts_with("https://") || url.starts_with("http://") {
            url.split("://").nth(1).unwrap_or(url).split('/').next().unwrap_or(url).to_string()
        } else {
            url.to_string()
        }
    }
    
    fn update_address_bar(&mut self) {
        if let Some(tab) = self.tabs.get(self.active_tab) {
            self.address_bar_text = tab.url.clone();
        }
    }
    
    fn render_menu_bar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New Tab").clicked() {
                        self.add_new_tab();
                        ui.close();
                    }
                    if ui.button("New Window").clicked() {
                        // TODO: Implement new window
                        ui.close();
                    }
                    ui.separator();
                    if ui.button("Exit").clicked() {
                        std::process::exit(0);
                    }
                });
                
                ui.menu_button("View", |ui| {
                    if ui.button("Bookmarks").clicked() {
                        self.show_bookmarks = !self.show_bookmarks;
                        ui.close();
                    }
                    if ui.button("History").clicked() {
                        self.show_history = !self.show_history;
                        ui.close();
                    }
                });
                
                ui.menu_button("Settings", |ui| {
                    if ui.button("Preferences").clicked() {
                        self.show_settings = !self.show_settings;
                        ui.close();
                    }
                });
                
                ui.menu_button("Help", |ui| {
                    if ui.button("About Ferrum").clicked() {
                        // TODO: Show about dialog
                        ui.close();
                    }
                });
            });
        });
    }
    
    fn render_navigation_bar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("nav_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                // Back button
                if ui.button("←").clicked() {
                    // TODO: Implement back navigation
                }
                
                // Forward button
                if ui.button("→").clicked() {
                    // TODO: Implement forward navigation
                }
                
                // Refresh button
                if ui.button("⟳").clicked() {
                    if let Some(tab) = self.tabs.get_mut(self.active_tab) {
                        tab.loading = true;
                        // TODO: Implement refresh
                        tab.loading = false;
                    }
                }
                
                // Address bar
                ui.add_space(10.0);
                let response = ui.add(egui::TextEdit::singleline(&mut self.address_bar_text).desired_width(f32::INFINITY));
                
                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    let url = self.address_bar_text.clone();
                    self.navigate_to(url);
                }
                
                // Bookmark button
                if ui.button("⭐").clicked() {
                    if let Some(tab) = self.tabs.get(self.active_tab) {
                        let bookmark = Bookmark {
                            title: tab.title.clone(),
                            url: tab.url.clone(),
                        };
                        if !self.bookmarks.iter().any(|b| b.url == bookmark.url) {
                            self.bookmarks.push(bookmark);
                        }
                    }
                }
                
                // Menu button
                if ui.button("☰").clicked() {
                    // TODO: Show menu
                }
            });
        });
    }
    
    fn render_tab_bar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("tab_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let mut tab_to_close = None;
                let mut new_active_tab = None;
                
                for (i, tab) in self.tabs.iter().enumerate() {
                    let is_active = i == self.active_tab;
                    
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            // Tab favicon (placeholder)
                            ui.label("🌐");
                            
                            // Tab title
                            let title = if tab.title.len() > 20 {
                                format!("{}...", &tab.title[..17])
                            } else {
                                tab.title.clone()
                            };
                            
                            let title_response = ui.selectable_label(is_active, title);
                            if title_response.clicked() {
                                new_active_tab = Some(i);
                            }
                            
                            // Loading indicator
                            if tab.loading {
                                ui.spinner();
                            }
                            
                            // Close button
                            if ui.small_button("×").clicked() {
                                tab_to_close = Some(i);
                            }
                        });
                    });
                }
                
                // New tab button
                if ui.button("+").clicked() {
                    self.add_new_tab();
                }
                
                // Apply changes after iteration
                if let Some(index) = new_active_tab {
                    self.active_tab = index;
                    self.update_address_bar();
                }
                
                // Close tab if requested
                if let Some(index) = tab_to_close {
                    self.close_tab(index);
                }
            });
        });
    }
    
    fn render_content_area(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(tab) = self.tabs.get(self.active_tab) {
                if tab.url == "about:blank" {
                    ui.vertical_centered(|ui| {
                        ui.add_space(200.0);
                        ui.heading("Welcome to Ferrum Browser");
                        ui.add_space(20.0);
                        ui.label("A modern browser built with Rust and egui");
                        ui.add_space(20.0);
                        
                        ui.horizontal(|ui| {
                            if ui.button("GitHub").clicked() {
                                self.navigate_to("https://github.com".to_string());
                            }
                            if ui.button("Rust").clicked() {
                                self.navigate_to("https://rust-lang.org".to_string());
                            }
                            if ui.button("egui").clicked() {
                                self.navigate_to("https://github.com/emilk/egui".to_string());
                            }
                        });
                    });
                } else {
                    // Web content area
                    ui.vertical(|ui| {
                        ui.heading(format!("Content for: {}", tab.url));
                        ui.separator();
                        ui.label("This is where the web content would be rendered.");
                        ui.label("You'll need to integrate a web engine like:");
                        ui.label("• webview2 (Windows)");
                        ui.label("• webkit2gtk (Linux)");
                        ui.label("• wkwebview (macOS)");
                        ui.label("• Or use a library like tauri-webview");
                    });
                }
            }
        });
    }
    
    fn render_sidebar(&mut self, ctx: &egui::Context) {
        if self.show_bookmarks {
            egui::SidePanel::left("bookmarks").show(ctx, |ui| {
                ui.heading("Bookmarks");
                ui.separator();
                
                let mut bookmark_to_navigate = None;
                let mut bookmark_to_remove = None;
                
                for (i, bookmark) in self.bookmarks.iter().enumerate() {
                    ui.horizontal(|ui| {
                        if ui.button(&bookmark.title).clicked() {
                            bookmark_to_navigate = Some(bookmark.url.clone());
                        }
                        if ui.small_button("×").clicked() {
                            bookmark_to_remove = Some(i);
                        }
                    });
                }
                
                // Apply changes after iteration
                if let Some(url) = bookmark_to_navigate {
                    self.navigate_to(url);
                }
                
                if let Some(i) = bookmark_to_remove {
                    self.bookmarks.remove(i);
                }
            });
        }
        
        if self.show_history {
            egui::SidePanel::right("history").show(ctx, |ui| {
                ui.heading("History");
                ui.separator();
                
                let mut url_to_navigate = None;
                for url in self.history.iter().rev() {
                    if ui.button(url).clicked() {
                        url_to_navigate = Some(url.clone());
                    }
                }
                
                if let Some(url) = url_to_navigate {
                    self.navigate_to(url);
                }
            });
        }
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let _ = frame;
        
        self.render_menu_bar(ctx);
        self.render_navigation_bar(ctx);
        self.render_tab_bar(ctx);
        self.render_sidebar(ctx);
        self.render_content_area(ctx);
        
        // Handle keyboard shortcuts
        if ctx.input(|i| i.key_pressed(egui::Key::T) && i.modifiers.ctrl) {
            self.add_new_tab();
        }
        
        if ctx.input(|i| i.key_pressed(egui::Key::W) && i.modifiers.ctrl) {
            self.close_tab(self.active_tab);
        }
    }
}