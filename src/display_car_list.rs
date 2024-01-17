use std::string::String;
use eframe::emath::Vec2;
use egui::{Image, Widget};
use regex::Regex;
use crate::ServerManager;

impl ServerManager {
    pub(crate) fn display_car_list(&mut self, ui: &mut egui::Ui) {
        if self.car_list_changed {
            self.update_config_car_list();
            self.car_list_changed = false;
        }
        egui::ScrollArea::both().id_source("car_list_scroll").show(ui, |ui| {
            let indices = self.car_indices.clone();
            for index in indices {
                let arr = self.car_textures.get(index);
                match arr {
                    Some(arr) => {
                        let tex = arr.get(0);
                        match tex {
                            Some(tex) => {
                                let image = Image::from_texture(tex).fit_to_exact_size(Vec2 { x: 120.0, y: 120.0 });
                                ui.horizontal(|ui| {
                                    if egui::Button::image(image).ui(ui).clicked() {
                                        println!("{}", index);
                                        self.remove_car(&index);
                                    }
                                    ui.label(self.available_car_list.get(index).unwrap());
                                });
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        });
    }

    fn update_config_car_list(&mut self) {
        let mut builder = String::from("");
        let mut i = 0;
        for car in &self.car_list {
            builder += car;
            if i != self.car_list.len() - 1 {
                builder += ";";
            }
            i += 1;
        }
        self.config.server.cars = builder;
    }

    fn remove_car(&mut self, index: &usize) {
        for i in 0..self.car_indices.len() {
            if self.car_indices.get(i).unwrap() == index {
                self.car_indices.remove(i);
                let car_name = self.available_car_list.get(*index);
                match car_name {
                    Some(car_name) => {
                        let regex = Regex::new(car_name).unwrap();
                        for j in 0..self.car_list.len() {
                            match regex.find(self.car_list.get(j).unwrap()) {
                                Some(res) => {
                                    if res.start() == 0 && res.end() == self.car_list.get(j).unwrap().len() {
                                        self.car_list.remove(j);
                                        break;
                                    }
                                }

                                _ => {}
                            }
                        }
                    }

                    None => {}
                }
                break;
            }
        }
        self.update_config_car_list();
    }

    pub(crate) fn add_car(&mut self, index: usize) {
        self.car_indices.push(index);
        self.car_list.push(String::from(self.available_car_list.get(index).unwrap()));
        self.update_config_car_list();
    }

    pub(crate) fn update_car_list_from_config(&mut self) {
        let regex = Regex::new(";").unwrap();
        let split = regex.split(&self.config.server.cars);
        self.car_indices = vec![];
        for name in split {
            let finder = Regex::new(name).unwrap();
            let mut i = 0;
            'inner: for car in &self.available_car_list {
                let found = finder.find(car);
                match found {
                    Some(_) => {
                        self.car_indices.push(i);
                        self.car_list.push(String::from(car));
                        break 'inner;
                    }

                    None => {}
                }
                i += 1;
            }
        }
    }
}