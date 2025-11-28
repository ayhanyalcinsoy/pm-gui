use eframe::egui;
use crate::gui::events::{AppEvent, EventManager};

#[derive(Default)]
pub struct Sidebar;

impl Sidebar {
    pub fn render(&self, ui: &mut egui::Ui, event_manager: &mut EventManager) {
        ui.vertical(|ui| {
            // Components section
            ui.heading("Components");
            let components = [
                ("All", 6501),
                    ("System", 200),
                    ("Development", 25),
                    ("Web", 21),
                    ("Office", 12),
                    ("Kde Desktop", 64),
                    ("Lxqt Desktop", 34),
                    ("Gnome Desktop", 23),
                    ("Xfce Desktop", 21),
                    ("Programming", 14),
                    ("Multimedia", 9),
                    ("Management Tools", 7),
            ];

            for (name, count) in components {
                let response = ui.selectable_label(false, format!("{} ({})", name, count));

                if response.clicked() {
                    event_manager.push(AppEvent::ComponentSelected(name.to_string()));
                }
            }

            ui.separator();

            // Categories section
            ui.heading("Categories");
            let categories = [
                ("Installed", 3127),
                    ("Updates", 15),
                    ("Explorer", 0),
            ];

            for (name, count) in categories {
                let response = ui.selectable_label(false, format!("{} ({})", name, count));

                if response.clicked() {
                    event_manager.push(AppEvent::CategorySelected(name.to_string()));
                }
            }

            ui.separator();

            // Repo messages
            ui.heading("Repo/Messages");
            let messages = [
                "Firefox is installed",
                "Libreoffice is updated",
                "Vlc is removed",
                "Main repo is updated",
            ];

            for message in messages {
                ui.label(format!("• {}", message));
            }
        });
    }
}
