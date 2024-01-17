use eframe::emath::Vec2;
use egui::{Image, TextureHandle, Widget};
use image::ImageFormat;
use regex::Regex;

use crate::ServerManager;

impl ServerManager {
    pub(crate) fn display_available_cars(&mut self, ui: &mut egui::Ui) {
        ui.text_edit_singleline(&mut self.available_car_filter);
        egui::ScrollArea::both().id_source("col3").show(ui, |ui| {
            if !self.discovered_cars {
                self.available_car_list = self.discover_cars();
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
        for car_folder in inner {
            match car_folder {
                Ok(car_folder) => {
                    let inner = std::fs::read_dir(car_folder.path());
                    if let Ok(inner) = inner {
                        for entry in inner {
                            match entry {
                                Ok(entry) => {
                                    if entry.file_name() == "ui" {
                                        let file_name = car_folder.file_name();
                                        result.push(file_name.to_str().unwrap().parse().unwrap());
                                    }
                                }

                                Err(e) => { println!("{}", e); }
                            }
                        }
                    }
                }
                Err(e) => {
                    println!("{}", e);
                }
            }
        }
        for s in &result {
            println!("{}", s);
        }
        result
    }

    fn generate_car_textures(&mut self, ui: &mut egui::Ui) {
        let path = self.assetto_corsa_path.clone().unwrap() + "\\content\\cars";
        let mut result: Vec<TextureHandle> = Vec::new();
        for car_name in &self.available_car_list {
            let skin_path = format!("{}\\{}\\skins", path.clone(), car_name);
            let skin_name = std::fs::read_dir(skin_path.clone());
            match skin_name {
                Ok(mut skin_name) => {
                    let skin_name = skin_name.nth(0).unwrap().unwrap().file_name().to_str().unwrap().to_string();
                    let image_path = format!("{}\\{}\\skins\\{}\\preview.jpg", path.clone(), car_name, skin_name);
                    let data = std::fs::read(image_path.clone());
                    match data {
                        Ok(data) => {
                            let image = image::load_from_memory_with_format(&data, ImageFormat::Jpeg);
                            match image {
                                Ok(image) => {
                                    println!("Successfully created car texture: {}", image_path);
                                    let texture = ServerManager::generate_texture(image, ui);
                                    result.push(texture);
                                }
                                Err(e) => {
                                    let image = image::load_from_memory_with_format(&std::fs::read(format!("{}\\launcher\\themes\\default\\graphics\\logo.png", self.assetto_corsa_path.clone().unwrap())).unwrap(), ImageFormat::Png).unwrap();
                                    let texture = ServerManager::generate_texture(image, ui);
                                    result.push(texture);
                                    println!("{}", e);
                                }
                            }
                        }
                        Err(e) => { // Error happens when there is a file inside of the skin folder or when a folder does not have a preview.jpg file
                            let image = image::load_from_memory_with_format(&std::fs::read(format!("{}\\launcher\\themes\\default\\graphics\\logo.png", self.assetto_corsa_path.clone().unwrap())).unwrap(), ImageFormat::Png).unwrap();
                            let texture = ServerManager::generate_texture(image, ui);
                            result.push(texture);
                            println!("{}", e)
                        }
                    }
                }
                _ => {
                    println!("Skins not found for {}", car_name);
                }
            }
        }
        for texture in result {
            let arr = vec![texture];
            self.car_textures.push(arr);
        }
    }

    fn _generate_skin_textures(&self, _ui: &mut egui::Ui, _index: usize) {}

    fn display_car_images(&mut self, ui: &mut egui::Ui) {
        let mut i = 0;
        let regex = Regex::new(&self.available_car_filter);
        match regex {
            Ok(regex) => {
                for arr in &self.car_textures.clone() {
                    if let Some(car_name) = self.available_car_list.clone().get(i) {
                        let check = regex.find(car_name); // Why doesn't regex.match() work? idk
                        if check.is_some() {
                            for texture in arr {
                                let image = Image::from_texture(texture).fit_to_exact_size(Vec2 { x: 120.0, y: 120.0 });
                                ui.horizontal(|ui| {
                                    if egui::Button::image(image).ui(ui).clicked() {
                                        self.add_car(i);
                                        println!("{}: {:?}", car_name, regex.find(car_name));
                                    }
                                    ui.label(car_name);
                                });
                            }
                        }
                        i += 1;
                    }
                }
            }

            Err(_e) => {}
        }
    }
}