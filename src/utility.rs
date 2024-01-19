use eframe::epaint::ColorImage;
use egui::{TextureHandle, Ui};
use image::{DynamicImage};
use crate::ServerManager;

impl ServerManager {

    // Probably could convert to macro instead
    pub(crate) fn generate_texture(image: DynamicImage, ui: &mut Ui) -> TextureHandle {
        let size = [image.width() as usize, image.height() as usize];
        let image_buffer = image.to_rgba8();
        let pixels = image_buffer.into_vec();
        let image = ColorImage::from_rgba_unmultiplied(size, &pixels);
        ui.ctx().load_texture("image", image, Default::default())
    }

    pub(crate) fn generate_placeholder(&self, ui: &mut Ui) -> TextureHandle {
        let image = image::open(format!("{}\\launcher\\themes\\default\\graphics\\logo.png", self.assetto_corsa_path.clone().unwrap())).unwrap();
        let size = [image.width() as usize, image.height() as usize];
        let image_buffer = image.to_rgba8();
        let pixels = image_buffer.into_vec();
        let image = ColorImage::from_rgba_unmultiplied(size, &pixels);
        ui.ctx().load_texture("placeholder_image", image, Default::default())
    }
}