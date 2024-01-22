use std::collections::HashMap;
use std::string::String;

use eframe::emath::Vec2;
use egui::{Image, Ui, Widget};

use crate::{Car, ServerManager};

impl ServerManager {
    pub(crate) fn display_car_list(&mut self, ui: &mut egui::Ui) {
        if self.car_list_changed {
            self.update_config_car_list();
            self.car_list_changed = false;
        }
        egui::ScrollArea::both().id_source("car_list_scroll").show(ui, |ui| {
            for (car_name, map) in &self.car_list.clone() {
                if map.is_empty() { // remove_car should deal with this, I don't know why this is even needed
                    self.car_list.remove(car_name);
                    continue;
                }
                ui.horizontal(|ui| {
                    let first_skin_name = map.iter().nth(0).unwrap();
                    let image = Image::from_texture(self.car_textures.get(car_name).unwrap().get(first_skin_name.0).unwrap()).fit_to_exact_size(Vec2 { x: 120.0, y: 120.0 });
                    if egui::Button::image(image).ui(ui).clicked() {
                        println!("those!")
                    }
                    ui.label(car_name.clone());
                    for (skin_name, count) in map {
                        let image = Image::from_texture(self.car_textures.get(car_name).unwrap().get(skin_name).unwrap()).fit_to_exact_size(Vec2 { x: 120.0, y: 120.0 });
                        for _i in 0..*count {
                            if egui::Button::image(image.clone()).ui(ui).clicked() {
                                self.remove_car(String::from(car_name), String::from(skin_name));
                                println!("{}, {}", car_name, skin_name)
                            }
                        }
                    }
                });
            }
        });
    }

    fn update_config_car_list(&mut self) {
        let mut builder = String::from("");
        for (i, map) in self.car_list.iter().enumerate() {
            builder += &map.1.iter().nth(0).unwrap().0;
            if i != self.car_list.len() - 1 {
                builder += ";";
            }
        }
        self.config.server.cars = builder;
    }

    // TODO: Add to car_list display
    fn remove_car(&mut self, name: String, skin: String) {
        let binding = self.car_list.get(&name).unwrap().clone();
        let num = binding.get(&skin).unwrap();
        if *num == 1 {
            self.car_list.get_mut(&name).unwrap().remove(&skin);

        } else {
            self.car_list.get_mut(&name).unwrap().insert(skin.clone(), *num - 1);
            if self.car_list.get_mut(&name).unwrap().is_empty() {
                self.car_list.remove(&name);
            }
        }
        for (i, entry) in self.entry_list.list.iter().enumerate() {
            if entry.model == name && entry.skin == skin {
                self.entry_list.list.remove(i);
                break;
            }
        }
        // self.update_config_car_list(); TODO: Need to update
    }


    /*
        TODO: Limit the amount of cars to the number of pits available (<track name>\ui\full\ui_track.json)
    */
    pub(crate) fn add_car(&mut self, car_name: &String, skin_name: String, from_list: bool) {
        if let Some(entry) = self.car_list.clone().get(car_name) {
            if let Some(num) = entry.get(&skin_name) {
                let _ = &self.car_list.get_mut(car_name).unwrap().remove(&skin_name);
                self.car_list.get_mut(car_name).unwrap().insert(skin_name.clone(), num + 1);
            } else {
                self.car_list.get_mut(car_name).unwrap().insert(skin_name.clone(), 1);
            }
        } else {
            let mut inner: HashMap<String, u8> =  Default::default();
            inner.insert(skin_name.clone(), 1);
            self.car_list.insert(car_name.clone(), inner);
        }
        if from_list {
            self.entry_list.list.push(Car { model: String::from(car_name), skin: String::from(&skin_name.clone()), ..Default::default() });
            for car in &self.entry_list.list {
                println!("{:?}", car);
            }
        }
    }

    pub(crate) fn update_from_config(&mut self, ui: &mut Ui) {
        for entry in &self.entry_list.list.clone() {
            self.add_car(&entry.model, String::from(&entry.skin), false);
            if self.car_textures.get(&entry.model).unwrap().len() <= 1 {
                self.generate_skin_textures(ui, &entry.model);
            }
        }
    }
}