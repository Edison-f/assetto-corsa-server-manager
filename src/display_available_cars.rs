use std::collections::HashMap;

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
        for car_name in &self.available_car_list {
            let skin_path = format!("{}\\{}\\skins", path.clone(), car_name);
            let skin_name = std::fs::read_dir(skin_path.clone());
            self.discover_skins(car_name);
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
                                    let mut map = HashMap::new();
                                    map.insert(skin_name, texture);
                                    self.car_textures.insert(car_name.clone(), map);
                                }
                                Err(e) => {
                                    let image = image::load_from_memory_with_format(&std::fs::read(format!("{}\\launcher\\themes\\default\\graphics\\logo.png", self.assetto_corsa_path.clone().unwrap())).unwrap(), ImageFormat::Png).unwrap();
                                    let texture = ServerManager::generate_texture(image, ui);
                                    let mut map = HashMap::new();
                                    map.insert(skin_name, texture);
                                    self.car_textures.insert(car_name.clone(), map);
                                    println!("{}", e);
                                }
                            }
                        }
                        Err(e) => { // Error happens when there is a file inside of the skin folder or when a folder does not have a preview.jpg file
                            let image = image::load_from_memory_with_format(&std::fs::read(format!("{}\\launcher\\themes\\default\\graphics\\logo.png", self.assetto_corsa_path.clone().unwrap())).unwrap(), ImageFormat::Png).unwrap();
                            let texture = ServerManager::generate_texture(image, ui);
                            let mut map = HashMap::new();
                            map.insert(skin_name, texture);
                            self.car_textures.insert(car_name.clone(), map);
                            println!("{}", e)
                        }
                    }
                }
                _ => {
                    println!("Skins not found for {}", car_name);
                }
            }
        }
    }

    fn discover_skins(&self, car_name: &String) -> Vec<String> {
        let path = self.assetto_corsa_path.clone().unwrap() + "\\content\\cars";
        let mut result: Vec<String> = Vec::new();
        let inner = std::fs::read_dir(format!("{}\\{}\\skins", path.clone(), car_name)).unwrap();
        for skin_folder in inner {
            match skin_folder {
                Ok(skin_folder) => {
                    let file_name = skin_folder.file_name();
                    result.push(file_name.to_str().unwrap().parse().unwrap());
                }
                Err(e) => {
                    println!("{}", e);
                }
            }
        }
        result
    }

    fn generate_skin_textures(&mut self, ui: &mut egui::Ui, target_car: &String) {
        let path = self.assetto_corsa_path.clone().unwrap() + "\\content\\cars\\" + target_car.as_str() + "\\skins\\";
        let mut result: Vec<TextureHandle> = Vec::new();
        let read_dir = std::fs::read_dir(path.clone()).unwrap();
        for entry in read_dir {
            match entry {
                Ok(entry) => {
                    let path = entry.path();
                    let image_path = path.clone().into_os_string().into_string().unwrap() + "\\preview.jpg";
                    let data = std::fs::read(image_path.clone());
                    match data {
                        Ok(data) => {
                            let image = image::load_from_memory(&data);
                            self.car_textures.get_mut(target_car).unwrap().insert(entry.file_name().into_string().unwrap(), ServerManager::generate_texture(image.unwrap(), ui));
                        }

                        Err(e) => {
                            println!("{}", e);
                        }
                    }
                }

                Err(e) => {
                    println!("{}", e);
                }
            }
        }
    }

    fn display_car_images(&mut self, ui: &mut egui::Ui) {
        let mut i = 0;
        let regex = Regex::new(&self.available_car_filter);
        match regex {
            Ok(regex) => {
                for car_name in &self.available_car_list.clone() {
                    let mut j = 0;
                    ui.horizontal(|ui| {
                        let pair = self.car_textures.get(car_name).unwrap().clone();
                        for (skin_name, texture) in pair {
                            // if j == 1 && !self.expand_available_skins.get(car_name).unwrap_or(&false) {
                            //     break;
                            // }
                            let texture = &texture.clone();
                            let image = Image::from_texture(texture).fit_to_exact_size(Vec2 { x: 120.0, y: 65.0 });
                            let button = egui::Button::image(image).ui(ui);
                            if button.clicked() {
                                self.add_car(car_name, skin_name.clone());
                                println!("{}: {:?}", car_name, skin_name);
                            }

                            if button.secondary_clicked() {
                                // let expand = self.expand_available_skins.get(car_name);
                                // match expand {
                                //     Some(expand) => {
                                //         self.expand_available_skins.insert(car_name.clone(), !expand);
                                //     }
                                //     None => {
                                //         self.expand_available_skins.insert(car_name.clone(), true);
                                //     }
                                // }
                                if self.car_textures.get(car_name).unwrap().len() == 1 {
                                    self.generate_skin_textures(ui, car_name);
                                    self.available_skins_list.insert(car_name.clone(), self.discover_skins(car_name));
                                }
                                println!("{}", i);
                            }
                            if j == 0 {
                                ui.label(car_name);
                            }
                            j += 1;
                        }
                    });

                    // if let Some(car_name) = self.available_car_list.clone().get(i) {
                    //     let check = regex.find(car_name); // Why doesn't regex.match() work? idk
                    //     if check.is_some() {
                    //         let mut j = 0;
                    //         ui.horizontal(|ui| {
                    //             for (_, texture) in arr.1 {
                    //                 if j == 1 && !self.expand_available_skins.get(car_name).unwrap_or(&false) {
                    //                     break;
                    //                 }
                    //                 let image = Image::from_texture(texture).fit_to_exact_size(Vec2 { x: 120.0, y: 65.0 });
                    //                 let button = egui::Button::image(image).ui(ui);
                    //                 if button.clicked() {
                    //                     self.add_car(i, j);
                    //                     println!("{}: {:?}", car_name, regex.find(car_name));
                    //                 }
                    //
                    //                 if button.secondary_clicked() {
                    //                     let expand = self.expand_available_skins.get(car_name);
                    //                     match expand {
                    //                         Some(expand) => {
                    //                             self.expand_available_skins.insert(car_name.clone(), !expand);
                    //                         }
                    //                         None => {
                    //                             self.expand_available_skins.insert(car_name.clone(), true);
                    //                         }
                    //                     }
                    //                     if self.car_textures.get(car_name).unwrap().len() == 1 {
                    //                         self.generate_skin_textures(ui, i);
                    //                         self.available_skins_list.insert(car_name.clone(), self.discover_skins(car_name));
                    //                     }
                    //                     println!("{}", i);
                    //                 }
                    //                 if j == 0 {
                    //                     ui.label(car_name);
                    //                 }
                    //                 j += 1;
                    //             }
                    //         });
                }
                i += 1;
            }
            Err(_e) => {}
        }
    }
}
