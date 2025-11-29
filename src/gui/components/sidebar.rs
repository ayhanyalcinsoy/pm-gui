use eframe::egui;
use crate::gui::events::{AppEvent, EventManager};

#[derive(Default)]
pub struct Sidebar;

impl Sidebar {
    pub fn render(&self, ui: &mut egui::Ui, event_manager: &mut EventManager) {
        ui.vertical(|ui| {
            // Components section
            ui.heading("Components");
            
            // Mock components yerine gerçek data kullanacağız
            // Bu kısım app'ten gelecek
            
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
                "Repository synchronized",
                "6366 packages available", 
                "197 components loaded",
            ];
            
            for message in messages {
                ui.label(format!("• {}", message));
            }
        });
    }
}
