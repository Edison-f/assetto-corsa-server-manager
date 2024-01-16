use eframe::emath::Vec2;
use eframe::epaint::ColorImage;
use egui::{Image, TextureHandle, Widget};
use image::{ImageFormat, ImageResult};
use regex::Regex;
use crate::ServerManager;

impl ServerManager {
    pub(crate) fn display_available_cars(&mut self, ui: &mut egui::Ui) {
        ui.text_edit_singleline(&mut self.car_filter);
        egui::ScrollArea::both().id_source("col3").show(ui, |ui| {
            if !self.discovered_cars {
                self.car_list = self.discover_cars();
                self.generate_car_textures(ui);
                self.discovered_cars = true;
            }
            self.display_car_images(ui);
        });
    }

    fn discover_cars(&self) -> Vec<String> {
        let path = self.assetto_corsa_path.clone().unwrap() + "\\content\\cars";
        let mut result: Vec<String> = Vec::new();
        let inner = std::fs::read_dir(path.clone()).unwrap();
        for outer_car_folder in inner {
            let file_name = outer_car_folder.unwrap().file_name();
            result.push(file_name.to_str().unwrap().parse().unwrap());
        }
        for s in &result {
            println!("{}", s);
        }
        return result;
    }

    fn generate_car_textures(&mut self, ui: &mut egui::Ui) {
        let path = self.assetto_corsa_path.clone().unwrap() + "\\content\\cars";
        let mut result: Vec<TextureHandle> = Vec::new();
        for car_name in &self.car_list {
            let skin_path = format!("{}\\{}\\skins", path.clone(), car_name);
            let skin_name = std::fs::read_dir(skin_path.clone());
            match skin_name {
                Ok(mut skin_name) => {
                    let skin_name = skin_name.nth(0).unwrap().unwrap().file_name().to_str().unwrap().to_string();
                    let image_path = format!("{}\\{}\\skins\\{}\\preview.jpg", path.clone(), car_name, skin_name);
                    let data = std::fs::read(image_path.clone());
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
                                    println!("Successfully created car texture: {}", image_path);
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
        let mut i = 0;
        let regex = Regex::new(&*self.car_filter);
        match regex {
            Ok(regex) => {
                for arr in &self.car_textures {
                    let car_name = self.car_list.get(i);
                    match car_name {
                        Some(car_name) => {
                            let check = regex.find(car_name); // Why doesn't regex.match() work? idk
                            match check {
                                Some(_) => {
                                    for texture in arr {
                                        let image = Image::from_texture(&*texture).fit_to_exact_size(Vec2 { x: 120.0, y: 120.0 });
                                        if egui::Button::image(image).ui(ui).clicked() {
                                            println!("{}: {:?}", car_name, regex.find(car_name));
                                        }
                                    }
                                }
                                _ => {}
                            }
                            i += 1;
                        }

                        None => {}
                    }
                }
            }

            Err(_e) => {}
        }
    }
}