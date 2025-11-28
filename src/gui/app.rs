use eframe::egui;
use crate::config::{AppSettings, Theme};
use crate::backend::xml_parser::{XmlParser, PackageInfo, Component, Source}; // Component ve Source'u da import et
use crate::gui::components::{Sidebar, PackageGrid, PackageDetails};
use crate::gui::components::settings_modal;
use crate::config::SettingsModalState;
use crate::gui::events::{AppEvent, EventManager};
use crate::gui::image_loader::ImageLoader;

#[derive(PartialEq)]
pub enum AppView {
    Welcome,
    PackageList,
}

pub struct PackageManagerApp {
    pub current_theme: Theme,
    pub settings: AppSettings,
    pub show_settings: bool,
    pub current_view: AppView,
    
    // Data
    pub packages: Vec<PackageInfo>,
    pub components: Vec<Component>,
    pub selected_component: String,
    pub selected_category: String,
    pub selected_package: Option<PackageInfo>,
    
    // UI State
    pub sidebar: Sidebar,
    pub package_grid: PackageGrid,
    pub package_details: PackageDetails,
    pub settings_modal: SettingsModalState,
    
    // Event system
    pub event_manager: EventManager,
    
    // Image loader
    pub image_loader: ImageLoader,
}

// ... geri kalan kod aynı kalacak ...

impl PackageManagerApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Sistem fontlarını yükle
        Self::setup_fonts(&cc.egui_ctx);
        
        // Initialize with default data (will be replaced with XML parsing)
        let packages = Vec::new();
        let components = Vec::new();
        
        let mut app = Self {
            current_theme: Theme::Light,
            settings: AppSettings::default(),
            show_settings: false,
            current_view: AppView::Welcome,
            packages,
            components,
            selected_component: "All".to_string(),
            selected_category: "Installed".to_string(),
            selected_package: None,
            sidebar: Sidebar::default(),
            package_grid: PackageGrid::default(),
            package_details: PackageDetails::default(),
            settings_modal: SettingsModalState::default(),
            event_manager: EventManager::new(),
            image_loader: ImageLoader::new(),
        };
        
        // Logoları yükle
        app.load_images(&cc.egui_ctx);
        
        app
    }

    /// Logoları yükle
    fn load_images(&mut self, ctx: &egui::Context) {
        // Mevcut çalışma dizinini al
        let current_dir = std::env::current_dir().unwrap_or_default();
        
        // Asset path'lerini oluştur
        let light_logo_path = current_dir.join("assets/pisi-logo-light.png");
        let dark_logo_path = current_dir.join("assets/pisi-logo-dark.png");
        let system_icon_path = current_dir.join("assets/package-manager-icon-systemtr.png");
        
        println!("Loading images from:");
        println!("  Light: {}", light_logo_path.display());
        println!("  Dark: {}", dark_logo_path.display());
        println!("  System: {}", system_icon_path.display());
        
        // Light tema logosu
        if let Ok(path_str) = light_logo_path.clone().into_os_string().into_string() {
            match self.image_loader.load_texture(ctx, &path_str, "pisi_logo_light") {
                Ok(()) => println!("✓ Successfully loaded pisi_logo_light"),
                Err(e) => println!("✗ Failed to load pisi_logo_light: {}", e),
            }
        } else {
            println!("✗ Invalid path for light logo");
        }
        
        // Dark tema logosu  
        if let Ok(path_str) = dark_logo_path.clone().into_os_string().into_string() {
            match self.image_loader.load_texture(ctx, &path_str, "pisi_logo_dark") {
                Ok(()) => println!("✓ Successfully loaded pisi_logo_dark"),
                Err(e) => println!("✗ Failed to load pisi_logo_dark: {}", e),
            }
        } else {
            println!("✗ Invalid path for dark logo");
        }
        
        // Sistem ikonu
        if let Ok(path_str) = system_icon_path.clone().into_os_string().into_string() {
            match self.image_loader.load_texture(ctx, &path_str, "system_tray_icon") {
                Ok(()) => println!("✓ Successfully loaded system_tray_icon"),
                Err(e) => println!("✗ Failed to load system_tray_icon: {}", e),
            }
        } else {
            println!("✗ Invalid path for system icon");
        }
    }

    /// Sistem fontlarını ve stillerini ayarla
    fn setup_fonts(ctx: &egui::Context) {
        let fonts = egui::FontDefinitions::default();
        ctx.set_fonts(fonts);
        
        // Stil ayarları - sistem temasına uyumlu
        let mut style = (*ctx.style()).clone();
        style.spacing.item_spacing = egui::vec2(8.0, 6.0);
        style.spacing.button_padding = egui::vec2(12.0, 6.0);
        ctx.set_style(style);
    }

    fn handle_events(&mut self) {
        while let Some(event) = self.event_manager.pop() {
            match event {
                AppEvent::CategorySelected(category) => {
                    self.selected_category = category;
                    self.current_view = AppView::PackageList;
                }
                AppEvent::ComponentSelected(component) => {
                    self.selected_component = component;
                    self.current_view = AppView::PackageList;
                }
                AppEvent::ShowSettings => {
                    self.show_settings = true;
                }
            }
        }
    }

    /// Ana logo render - welcome screen için
    fn render_main_logo(&self, ui: &mut egui::Ui) {
        let logo_name = if self.current_theme == Theme::Light {
            "pisi_logo_light"
        } else {
            "pisi_logo_dark"
        };
        
        if let Some(texture) = self.image_loader.get_texture(logo_name) {
            // Logo boyutunu ayarla - image_size kullan
            ui.add(egui::Image::new(texture).max_size(egui::vec2(200.0, 200.0)));
        } else {
            // Fallback text logo
            ui.vertical_centered(|ui| {
                ui.heading("🦊 Pisi GNU/Linux");
                ui.add_space(10.0);
                ui.label("Paket Yöneticisi");
            });
        }
    }

    /// Küçük logo render - header için
    fn render_small_logo(&self, ui: &mut egui::Ui, size: f32) {
        let logo_name = if self.current_theme == Theme::Light {
            "pisi_logo_dark" // Light temada dark logo daha görünür olur
        } else {
            "pisi_logo_light" // Dark temada light logo
        };
        
        if let Some(texture) = self.image_loader.get_texture(logo_name) {
            ui.add(egui::Image::new(texture).max_width(size).max_height(size));
        } else {
            // Fallback emoji
            ui.label("🦊");
        }
    }

    // Debug için texture durumunu göster
    fn render_texture_debug_info(&self, ui: &mut egui::Ui) {
        if cfg!(debug_assertions) {
            ui.collapsing("Texture Debug Info", |ui| {
                let loaded_textures = self.image_loader.get_loaded_textures();
                ui.label(format!("Loaded textures: {}", loaded_textures.len()));
                for texture_name in loaded_textures {
                    ui.label(format!("✓ {}", texture_name));
                }
                
                // Check specific textures
                let textures_to_check = ["pisi_logo_light", "pisi_logo_dark", "system_tray_icon"];
                for texture_name in textures_to_check {
                    let status = if self.image_loader.has_texture(texture_name) {
                        "✓ Loaded"
                    } else {
                        "✗ Failed"
                    };
                    ui.label(format!("{}: {}", texture_name, status));
                }
            });
        }
    }

    fn render_quick_stats(&self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.heading("6501");
                ui.label("Toplam Paket");
            });
            
            ui.add_space(30.0);
            
            ui.vertical(|ui| {
                ui.heading("15");
                ui.label("Güncelleme");
            });
            
            ui.add_space(30.0);
            
            ui.vertical(|ui| {
                ui.heading("3127");
                ui.label("Kurulu Paket");
            });
        });
    }
    
    fn render_quick_actions(&self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("📦 Tüm Paketleri Görüntüle").clicked() {
                // Bu event'leri handle_events'te yakalayacağız
                println!("Tüm paketler butonuna tıklandı");
            }
            
            if ui.button("🔄 Güncellemeleri Kontrol Et").clicked() {
                println!("Güncellemeler butonuna tıklandı");
            }
        });
    }
    pub fn load_packages_from_xml(&mut self) {
        if !self.packages.is_empty() {
            return; // Zaten yüklü
        }

        println!("Loading packages from local Pisi repository...");
        
        match XmlParser::load_pisi_index() {
            Ok(packages) => {
                self.packages = packages;
                self.components = XmlParser::parse_components(&self.packages);
                println!("Successfully loaded {} packages, {} components", 
                         self.packages.len(), self.components.len());
                
                // Debug: İlk birkaç paketi göster
                for pkg in self.packages.iter().take(5) {
                    println!("  - {}: {} ({})", pkg.name, pkg.summary, pkg.part_of);
                }
            }
            Err(e) => {
                println!("Failed to load Pisi index: {}. Using mock data.", e);
                self.create_mock_data();
            }
        }
    }

    /// Mock data fallback - sadece Pisi index yoksa
    fn create_mock_data(&mut self) {
        println!("Creating mock data for demonstration...");
        
        self.packages = vec![
            PackageInfo {
                name: "firefox".to_string(),
                summary: "Mozilla Firefox Web Browser".to_string(),
                description: "Fast and secure web browser".to_string(),
                version: "115.0".to_string(),
                release: 1,
                license: "MPL-2.0".to_string(),
                part_of: "web".to_string(),
                package_size: 97_000_000,
                installed_size: 245_000_000,
                package_format: "1.0".to_string(),
                distribution: "PisiLinux".to_string(),
                distribution_release: "2.0".to_string(),
                architecture: "x86_64".to_string(),
                source: Some(Source {
                    name: "firefox".to_string(),
                    homepage: "https://www.mozilla.org/firefox/".to_string(),
                }),
                history: vec![],
                dependencies: vec![],
            },
            PackageInfo {
                name: "libreoffice".to_string(),
                summary: "LibreOffice Office Suite".to_string(),
                description: "Complete office productivity suite".to_string(),
                version: "7.5.0".to_string(),
                release: 1,
                license: "MPL-2.0".to_string(),
                part_of: "office".to_string(),
                package_size: 245_000_000,
                installed_size: 512_000_000,
                package_format: "1.0".to_string(),
                distribution: "PisiLinux".to_string(),
                distribution_release: "2.0".to_string(),
                architecture: "x86_64".to_string(),
                source: Some(Source {
                    name: "libreoffice".to_string(),
                    homepage: "https://www.libreoffice.org/".to_string(),
                }),
                history: vec![],
                dependencies: vec![],
            },
        ];
        
        self.components = XmlParser::parse_components(&self.packages);
    }
}

impl eframe::App for PackageManagerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Apply theme
        self.current_theme.apply(ctx);
        
        // Handle pending events
        self.handle_events();
        
        // Load data if needed
        self.load_packages_from_xml();

        // Top panel with header
        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            self.render_header(ui);
        });

        // Sidebar
        egui::SidePanel::left("sidebar")
            .resizable(false)
            .min_width(250.0)
            .show(ctx, |ui| {
                self.sidebar.render(ui, &mut self.event_manager);
            });

        // Package details panel
        egui::SidePanel::right("details")
            .resizable(false)
            .min_width(300.0)
            .show(ctx, |ui| {
                let app_ref = &*self;
                self.package_details.render(ui, app_ref);
            });

        // Main content area
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.current_view {
                AppView::Welcome => self.render_welcome_screen(ui),
                AppView::PackageList => {
                    let app_ref = &*self;
                    self.package_grid.render(ui, app_ref);
                }
            }
        });

        // Settings modal
        if self.show_settings {
            settings_modal::SettingsModal::render(ctx, self);
        }
    }
}

impl PackageManagerApp {
    fn render_header(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            // Sol tarafta küçük logo ve başlık
            ui.horizontal(|ui| {
                // Küçük logo - temaya göre değişen
                self.render_small_logo(ui, 24.0);
                ui.heading("Pisi Paket Yöneticisi");
            });
            
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // Theme toggle
                let theme_text = if self.current_theme == Theme::Light {
                    "🌙 Dark"
                } else {
                    "☀️ Light"
                };
                
                if ui.button(theme_text).clicked() {
                    self.current_theme = self.current_theme.next();
                }
                
                // Settings button
                if ui.button("⚙️ Ayarlar").clicked() {
                    self.show_settings = true;
                }
            });
        });
    }

    fn render_welcome_screen(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(60.0);
            
            // Pisi Logo - temaya göre değişen
            self.render_main_logo(ui);
            
            ui.add_space(30.0);
            
            // Welcome message
            ui.heading("Pisi GNU/Linux Paket Yöneticisi");
            ui.add_space(10.0);
            ui.label("Sol taraftaki kategorilerden birini seçerek paketleri görüntüleyebilirsiniz.");
            
            ui.add_space(40.0);
            
            // Quick stats
            self.render_quick_stats(ui);
            
            ui.add_space(40.0);
            
            // Quick actions
            self.render_quick_actions(ui);

            // Debug info (sadece debug build'de görünür)
            self.render_texture_debug_info(ui);
        });
    }
}
