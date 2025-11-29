use eframe::egui;
use crate::gui::app::PackageManagerApp;
use crate::backend::package_manager::PackageManager;
use crate::backend::xml_parser::PackageInfo;

#[derive(Default)]
pub struct PackageGrid;

impl PackageGrid {
    pub fn render(&self, ui: &mut egui::Ui, app: &PackageManagerApp) {
        ui.vertical(|ui| {
            // Category header
            ui.horizontal(|ui| {
                ui.heading(&format!("{} Packages", app.selected_category));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(format!("{} packages found", self.get_filtered_packages(app).len()));
                });
            });
            
            ui.separator();
            
            // Package grid
            egui::ScrollArea::vertical().show(ui, |ui| {
                let filtered_packages = self.get_filtered_packages(app);
                
                if filtered_packages.is_empty() {
                    ui.vertical_centered(|ui| {
                        ui.heading("No packages found");
                        ui.label("Try selecting a different category or component");
                    });
                } else {
                    // Use columns for package grid
                    let columns = 3;
                    let packages_per_column = (filtered_packages.len() + columns - 1) / columns;
                    
                    ui.columns(columns, |columns| {
                        for (col_index, column) in columns.iter_mut().enumerate() {
                            column.vertical(|ui| {
                                let start_index = col_index * packages_per_column;
                                let end_index = std::cmp::min(start_index + packages_per_column, filtered_packages.len());
                                
                                for i in start_index..end_index {
                                    if let Some(package) = filtered_packages.get(i) {
                                        self.render_package_card(ui, package);
                                    }
                                }
                            });
                        }
                    });
                }
            });
        });
    }
    
    fn get_filtered_packages<'a>(&self, app: &'a PackageManagerApp) -> Vec<&'a PackageInfo> {
        app.packages.iter()
            .filter(|pkg| {
                // Component filtreleme
                let component_match = if app.selected_component == "All" {
                    true
                } else {
                    pkg.part_of == app.selected_component
                };
                
                component_match
            })
            .collect()
    }
    
    fn render_package_card(&self, ui: &mut egui::Ui, package: &PackageInfo) {
        egui::Frame::group(ui.style()).show(ui, |ui| {
            ui.vertical(|ui| {
                // Package header
                ui.horizontal(|ui| {
                    // Paket ikonu
                    let icon = self.get_package_icon(&package.name);
                    ui.label(icon);
                    
                    ui.vertical(|ui| {
                        ui.heading(&package.name);
                        if !package.summary.is_empty() {
                            ui.label(&package.summary);
                        }
                    });
                });
                
                // Version info
                ui.label(format!("Version: {}", package.version));
                
                // Component info
                let formatted_component = Self::format_component_name(&package.part_of);
                ui.label(format!("Category: {}", formatted_component));
                
                // Size info - MB cinsinden göster
                let size_mb = package.package_size / 1_000_000;
                if size_mb > 0 {
                    ui.label(format!("Download Size: {} MB", size_mb));
                }
                
                // License
                if !package.license.is_empty() {
                    ui.label(format!("License: {}", package.license));
                }
                
                // Action buttons
                ui.horizontal(|ui| {
                    if self.is_package_installed(&package.name) {
                        if self.has_package_update(&package.name) {
                            if ui.button("🔄 Update").clicked() {
                                // Update işlemi
                            }
                        }
                        
                        if ui.button("🗑️ Remove").clicked() {
                            // Remove işlemi
                        }
                    } else {
                        if ui.button("📥 Install").clicked() {
                            // Install işlemi
                        }
                    }
                });
            });
        });
    }
    
    fn format_component_name(raw_name: &str) -> String {
        // Basit formatlama - daha sonra xml_parser'daki fonksiyonu kullanabiliriz
        raw_name.replace('.', " - ")
            .split(' ')
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                }
            })
            .collect::<Vec<String>>()
            .join(" ")
    }
    
    // Geçici implementasyonlar - bunları daha sonra gerçek Pisi komutlarıyla değiştireceğiz
    fn is_package_installed(&self, _package_name: &str) -> bool {
        // Geçici: Her zaman false döndür
        false
    }
    
    fn has_package_update(&self, _package_name: &str) -> bool {
        // Geçici: Her zaman false döndür  
        false
    }
    
    fn get_package_icon(&self, package_name: &str) -> &'static str {
        // Basit paket ikonu mapping
        match package_name {
            "firefox" => "🌐",
            "libreoffice" => "📊",
            "vlc" => "🎬",
            "gimp" => "🎨",
            "thunderbird" => "📧",
            "konsole" => "💻",
            _ => "📦",
        }
    }
}
