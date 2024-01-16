use eframe::emath::Vec2;
use eframe::epaint::ColorImage;
use egui::{Image, TextureHandle};
use image::{ImageFormat, ImageResult};
use crate::ServerManager;

impl ServerManager {
    pub(crate) fn display_cars(&mut self, ui: &mut egui::Ui) {
        if !self.discovered_cars {
            self.car_list = self.discover_cars();
            self.generate_car_textures(ui);
            self.discovered_cars = true;
        }
        self.display_car_images(ui);
    }

    fn search_bar(&mut self, ui: egui::Ui) {

    }

    fn discover_cars(&self) -> Vec<String> {
        let path = self.assetto_corsa_path.clone().unwrap() + "\\content\\cars";
        let mut result: Vec<String> = Vec::new();
        let inner = std::fs::read_dir(path.clone()).unwrap();
        for outer_car_folder in inner {
            let file_name = outer_car_folder.unwrap().file_name();
            result.push(file_name.to_str().unwrap().parse().unwrap());
        }
        return result;
    }

    fn generate_car_textures(&mut self, ui: &mut egui::Ui) {
        let path = self.assetto_corsa_path.clone().unwrap() + "\\content\\cars";
        let mut result: Vec<TextureHandle> = Vec::new();
        for car_name in &self.car_list {
            self.car_textures.push(Vec::new());
            let skin_path = format!("{}\\{}\\skins", path.clone(), car_name);
            let skin_name = std::fs::read_dir(skin_path.clone());
            match skin_name {
                Ok(mut skin_name) => {
                    let skin_name = skin_name.nth(0).unwrap().unwrap().file_name().to_str().unwrap().to_string();
                    let image_path = format!("{}\\{}\\skins\\{}\\preview.jpg", path.clone(), car_name, skin_name);
                    let data = std::fs::read(image_path);
                    match data {
                        Ok(data) => {
                            let image = image::load_from_memory_with_format(&*data, ImageFormat::Jpeg);
                            match image {
                                Ok(image) => {
                                    let size = [image.width() as usize, image.height() as usize];
                                    let image_buffer = image.to_rgba8();
                                    let pixels = image_buffer.into_vec();
                                    let image = ColorImage::from_rgba_unmultiplied(size, &pixels);
                                    let texture = ui.ctx().load_texture("car", image, Default::default());
                                    result.push(texture);
                                }
                                Err(e) => {
                                    println!("{}", e);
                                }
                            }
                        }
                        Err(e) => { println!("{}", e) }
                    }
                }
                _ => {}
            }
        }
        for texture in result {
            let mut arr = Vec::new();
            arr.push(texture);
            self.car_textures.push(arr);
        }
    }

    fn generate_skin_textures(&self, ui: &mut egui::Ui, index: usize) {}

    fn display_car_images(&self, ui: &mut egui::Ui) {
        for arr in &self.car_textures {
            for texture in arr {
                let image = Image::from_texture(&*texture).fit_to_exact_size(Vec2 { x: 120.0, y: 120.0 });
                ui.add(image);
            }
        }
    }
}