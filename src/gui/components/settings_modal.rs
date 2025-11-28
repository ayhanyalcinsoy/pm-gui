use eframe::egui;
use crate::gui::app::PackageManagerApp;
use crate::config::{Repository, SettingsTab};

#[derive(Default)]
pub struct SettingsModal;

impl SettingsModal {
    pub fn render(ctx: &egui::Context, app: &mut PackageManagerApp) {
        let mut show_settings = app.show_settings;

        egui::Window::new("Package Manager Settings")
        .open(&mut show_settings)
        .resizable(true)
        .default_width(600.0)
        .show(ctx, |ui| {
            let mut current_tab = app.settings_modal.current_tab.clone();

            ui.horizontal(|ui| {
                // Left menu - sadece 4 ana sekme
                ui.vertical(|ui| {
                    ui.selectable_value(&mut current_tab, SettingsTab::General, "General");
                    ui.selectable_value(&mut current_tab, SettingsTab::Cache, "Package Cache");
                    ui.selectable_value(&mut current_tab, SettingsTab::Repos, "Package Repos");
                    ui.selectable_value(&mut current_tab, SettingsTab::Proxy, "Proxy Settings");
                });

                ui.separator();

                // Right content
                ui.vertical(|ui| {
                    match current_tab {
                        SettingsTab::General => Self::render_general_settings(ui, app),
                            SettingsTab::Cache => Self::render_cache_settings(ui, app),
                            SettingsTab::Repos => Self::render_repo_settings(ui, app),
                            SettingsTab::Proxy => Self::render_proxy_settings(ui, app),
                    }
                });
            });

            // Update the tab in the app state
            app.settings_modal.current_tab = current_tab;
        });

        // Update show_settings flag
        app.show_settings = show_settings;
    }

    fn render_general_settings(ui: &mut egui::Ui, app: &mut PackageManagerApp) {
        ui.heading("General Settings");

        // System Tray
        ui.checkbox(&mut app.settings.system_tray_enabled, "Enable system tray icon");

        ui.separator();

        // Package List View Settings
        ui.heading("Package List View");
        ui.checkbox(&mut app.settings.package_list.show_only_desktop_apps, "Show only desktop applications");
        ui.checkbox(&mut app.settings.package_list.show_components_info, "Show components info");
        ui.checkbox(&mut app.settings.package_list.show_package_type_labels, "Show labels of package type");

        ui.separator();

        // Update Settings
        ui.heading("Update Settings");
        ui.checkbox(&mut app.settings.update.check_updates, "Check updates");

        ui.horizontal(|ui| {
            let mut enable_interval = app.settings.update.update_check_interval > 0;
            ui.checkbox(&mut enable_interval, "Enable update check interval");
            if enable_interval {
                ui.add(egui::DragValue::new(&mut app.settings.update.update_check_interval).suffix(" m"));
            } else {
                app.settings.update.update_check_interval = 0;
            }
        });

        ui.checkbox(&mut app.settings.update.install_updates_automatically, "Install updates automatically");
    }

    fn render_cache_settings(ui: &mut egui::Ui, app: &mut PackageManagerApp) {
        ui.heading("Package Cache");
        ui.checkbox(&mut app.settings.cache.use_disk_cache, "Use disk cache for downloaded packages");

        ui.horizontal(|ui| {
            ui.label("Cache size (MB):");
            ui.add(egui::DragValue::new(&mut app.settings.cache.cache_size_mb));
        });

        ui.horizontal(|ui| {
            ui.label("Cache directory:");
            ui.text_edit_singleline(&mut app.settings.cache.cache_directory);
        });

        if ui.button("Clear Cache").clicked() {
            // Clear cache implementation
        }

        ui.separator();
        ui.checkbox(&mut app.settings.cache.use_disk_cache, "Use bandwidth limit");
    }

    fn render_repo_settings(ui: &mut egui::Ui, app: &mut PackageManagerApp) {
        ui.heading("Package Repositories");

        // Repository table
        ui.horizontal(|ui| {
            ui.label("Enable");
            ui.label("Name");
            ui.label("URL");
        });

        ui.separator();

        for repo in &mut app.settings.repositories {
            ui.horizontal(|ui| {
                ui.checkbox(&mut repo.enabled, "");
                ui.label(&repo.name);
                ui.label(&repo.url);
            });
        }

        ui.separator();

        // Repository management buttons
        ui.horizontal(|ui| {
            if ui.button("Add Repo").clicked() {
                app.settings.repositories.push(Repository {
                    enabled: true,
                    name: "New Repo".to_string(),
                                               url: "https://example.com/pisi-index.xml.xz".to_string(),
                });
            }

            if ui.button("Remove Repo").clicked() {
                if !app.settings.repositories.is_empty() {
                    app.settings.repositories.pop();
                }
            }

            if ui.button("Move Up").clicked() {
                // Move repo up implementation
            }

            if ui.button("Move Down").clicked() {
                // Move repo down implementation
            }
        });
    }

    fn render_proxy_settings(ui: &mut egui::Ui, app: &mut PackageManagerApp) {
        ui.heading("Proxy Settings");

        ui.checkbox(&mut app.settings.proxy.use_proxy, "Use proxy");

        if app.settings.proxy.use_proxy {
            ui.label("HTTP Proxy:");
            ui.text_edit_singleline(&mut app.settings.proxy.http_proxy);

            ui.label("HTTPS Proxy:");
            ui.text_edit_singleline(&mut app.settings.proxy.https_proxy);

            ui.label("FTP Proxy:");
            ui.text_edit_singleline(&mut app.settings.proxy.ftp_proxy);

            ui.label("Domain:");
            ui.text_edit_singleline(&mut app.settings.proxy.domain);

            ui.separator();

            ui.heading("Identity Verification");
            ui.label("Username:");
            ui.text_edit_singleline(&mut app.settings.proxy.username);

            ui.label("Password:");
            ui.text_edit_singleline(&mut app.settings.proxy.password);

            ui.checkbox(&mut app.settings.proxy.use_http_proxy_for_all, "Use HTTP Proxy for All");
        }
    }
}
