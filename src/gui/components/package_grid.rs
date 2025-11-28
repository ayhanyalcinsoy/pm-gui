use eframe::egui;
use crate::gui::app::PackageManagerApp;
use crate::backend::package_manager::PackageManager;

#[derive(Default)]
pub struct PackageGrid;

impl PackageGrid {
    pub fn render(&self, ui: &mut egui::Ui, app: &PackageManagerApp) {
        ui.vertical(|ui| {
            // Category header
            ui.horizontal(|ui| {
                ui.heading(&format!("{} Packages", app.selected_category));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(format!("{} packages found", 6)); // Mock count
                });
            });

            ui.separator();

            // Package grid
            egui::ScrollArea::vertical().show(ui, |ui| {
                // Use columns with explicit handling
                let columns = 3;
                let total_packages = 6; // Mock number
                let packages_per_column = (total_packages + columns - 1) / columns;

                ui.columns(columns, |columns| {
                    for (col_index, column) in columns.iter_mut().enumerate() {
                        column.vertical(|ui| {
                            // Render packages for this column
                            for i in 0..packages_per_column {
                                let package_index = col_index * packages_per_column + i;
                                if package_index < total_packages {
                                    self.render_package_card(ui, package_index);
                                }
                            }
                        });
                    }
                });
            });
        });
    }

    fn render_package_card(&self, ui: &mut egui::Ui, package_index: usize) {
        // Mock package data based on index
        let package = match package_index {
            0 => MockPackage {
                name: "Firefox".to_string(),
                description: "Web browser".to_string(),
                version: "115.0".to_string(),
                installed_version: Some("114.0".to_string()),
                icon: "🦊",
                is_installed: true,
                has_update: true,
            },
            1 => MockPackage {
                name: "Libreoffice".to_string(),
                description: "Office Software".to_string(),
                version: "7.5.0".to_string(),
                installed_version: Some("7.5.0".to_string()),
                icon: "📊",
                is_installed: true,
                has_update: false,
            },
            2 => MockPackage {
                name: "Vlc".to_string(),
                description: "Multimedia Player".to_string(),
                version: "3.0.18".to_string(),
                installed_version: None,
                icon: "🎬",
                is_installed: false,
                has_update: false,
            },
            3 => MockPackage {
                name: "Konsole".to_string(),
                description: "KDE Terminal".to_string(),
                version: "22.12.0".to_string(),
                installed_version: Some("22.12.0".to_string()),
                icon: "💻",
                is_installed: true,
                has_update: false,
            },
            4 => MockPackage {
                name: "Gimp".to_string(),
                description: "Image Editor".to_string(),
                version: "2.10.32".to_string(),
                installed_version: None,
                icon: "🎨",
                is_installed: false,
                has_update: false,
            },
            _ => MockPackage {
                name: "Thunderbird".to_string(),
                description: "Email Client".to_string(),
                version: "115.0".to_string(),
                installed_version: Some("114.0".to_string()),
                icon: "📧",
                is_installed: true,
                has_update: true,
            },
        };

        egui::Frame::group(ui.style()).show(ui, |ui| {
            ui.vertical(|ui| {
                // Package header
                ui.horizontal(|ui| {
                    ui.label(package.icon);

                    ui.vertical(|ui| {
                        ui.heading(&package.name);
                        ui.label(&package.description);
                    });
                });

                // Version info
                ui.label(format!("Version: {}", package.version));

                if let Some(installed) = &package.installed_version {
                    ui.label(format!("Installed: {}", installed));
                }

                // Action buttons
                ui.horizontal(|ui| {
                    if package.is_installed {
                        if package.has_update {
                            if ui.button("🔄 Update").clicked() {
                                if let Err(e) = PackageManager::update_package(&package.name) {
                                    eprintln!("Update error: {}", e);
                                }
                            }
                        }

                        if ui.button("🗑️ Remove").clicked() {
                            if let Err(e) = PackageManager::remove_package(&package.name) {
                                eprintln!("Remove error: {}", e);
                            }
                        }
                    } else {
                        if ui.button("📥 Install").clicked() {
                            if let Err(e) = PackageManager::install_package(&package.name) {
                                eprintln!("Install error: {}", e);
                            }
                        }
                    }
                });
            });
        });
    }
}

// Mock package structure for demonstration
struct MockPackage {
    name: String,
    description: String,
    version: String,
    installed_version: Option<String>,
    icon: &'static str,
    is_installed: bool,
    has_update: bool,
}
