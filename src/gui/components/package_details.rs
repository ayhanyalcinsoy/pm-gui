use eframe::egui;
use crate::gui::app::PackageManagerApp;
use crate::backend::package_manager::PackageManager;
use crate::gui::app::AppView;  // AppView'ı import ediyoruz

#[derive(Default)]
pub struct PackageDetails;

impl PackageDetails {
    pub fn render(&self, ui: &mut egui::Ui, app: &PackageManagerApp) {
        ui.vertical(|ui| {
            ui.heading("Package Details");

            if app.current_view == AppView::Welcome {
                ui.separator();
                ui.label("Hoş geldiniz!");
                ui.label("Sol taraftan bir kategori seçerek paketleri görüntüleyebilirsiniz.");
                ui.add_space(10.0);
                ui.label("Seçili kategori:");
                ui.heading(&app.selected_category);
            } else {
                // Show package details when in package list view
                ui.separator();
                ui.label("Name: Firefox");
                ui.label("Description: Web browser");
                ui.label("Current Version: 115.0");
                ui.label("Installed Version: 114.0");
                ui.label("Repository: main");
                ui.label("Size: 97 MB");

                ui.separator();

                // Action buttons
                ui.horizontal(|ui| {
                    if ui.button("Update Package").clicked() {
                        if let Err(e) = PackageManager::update_package("firefox") {
                            eprintln!("Update error: {}", e);
                        }
                    }

                    if ui.button("Remove Package").clicked() {
                        if let Err(e) = PackageManager::remove_package("firefox") {
                            eprintln!("Remove error: {}", e);
                        }
                    }
                });
            }

            ui.separator();

            // Info panel
            ui.label("Bilgi");
            ui.label(format!("Total Packages: {}", app.packages.len()));
            ui.label("There are 15 packages to update");
        });
    }
}
