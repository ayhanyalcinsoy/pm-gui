mod config;
mod backend;
mod gui;

use eframe::egui;
use gui::PackageManagerApp;

fn main() -> eframe::Result<()> {
    // Asset dosyalarının varlığını kontrol et
    check_assets();
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_title("Pisi GNU/Linux Software Center"),
        
        follow_system_theme: false,
        default_theme: eframe::Theme::Dark,
        
        ..Default::default()
    };

    eframe::run_native(
        "Pisi Package Manager",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
            Box::new(PackageManagerApp::new(cc))
        }),
    )
}

fn check_assets() {
    let asset_files = [
        "assets/pisi-logo-light.png",
        "assets/pisi-logo-dark.png", 
        "assets/package-manager-icon-systemtr.png",
    ];
    
    println!("Checking asset files...");
    for asset in &asset_files {
        if std::path::Path::new(asset).exists() {
            println!("✓ Found: {}", asset);
        } else {
            println!("✗ Missing: {}", asset);
        }
    }
}
