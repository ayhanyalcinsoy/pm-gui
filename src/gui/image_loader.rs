use eframe::egui;
use std::collections::HashMap;

pub struct ImageLoader {
    textures: HashMap<String, egui::TextureHandle>,
    loaded_textures: HashMap<String, bool>,
}

impl ImageLoader {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
            loaded_textures: HashMap::new(),
        }
    }
    
    pub fn load_texture(&mut self, ctx: &egui::Context, path: &str, name: &str) -> Result<(), String> {
        // Check if file exists first
        if !std::path::Path::new(path).exists() {
            self.loaded_textures.insert(name.to_string(), false);
            return Err(format!("Image file not found: {}", path));
        }
        
        // Load image from file
        let image_bytes = std::fs::read(path)
            .map_err(|e| format!("Failed to read image {}: {}", path, e))?;
        
        let image = self::load_image_from_bytes(&image_bytes)
            .map_err(|e| format!("Failed to load image {}: {}", path, e))?;
        
        let texture = ctx.load_texture(name, image, egui::TextureOptions::default());
        self.textures.insert(name.to_string(), texture);
        self.loaded_textures.insert(name.to_string(), true);
        
        println!("Successfully loaded texture: {} from {}", name, path);
        Ok(())
    }
    
    pub fn get_texture(&self, name: &str) -> Option<&egui::TextureHandle> {
        self.textures.get(name)
    }
    
    pub fn has_texture(&self, name: &str) -> bool {
        self.loaded_textures.get(name).copied().unwrap_or(false)
    }
    
    pub fn get_loaded_textures(&self) -> Vec<&str> {
        self.loaded_textures
            .iter()
            .filter(|(_, &loaded)| loaded)
            .map(|(name, _)| name.as_str())
            .collect()
    }
}

fn load_image_from_bytes(image_bytes: &[u8]) -> Result<egui::ColorImage, image::ImageError> {
    let image = image::load_from_memory(image_bytes)?;
    let size = [image.width() as usize, image.height() as usize];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.into_raw();
    Ok(egui::ColorImage::from_rgba_unmultiplied(size, &pixels))
}
