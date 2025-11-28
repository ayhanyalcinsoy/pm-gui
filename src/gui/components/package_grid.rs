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

    [dependencies]
eframe = "0.27"
egui = "0.27"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
roxmltree = "0.19"
anyhow = "1.0"
image = { version = "0.24", default-features = false, features = ["png"] }
# minreq kaldırıldı - artık HTTP isteği yapmıyoruz
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
